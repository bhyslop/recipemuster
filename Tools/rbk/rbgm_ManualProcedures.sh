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

  bug_section  "Manual Payor OAuth Establishment Procedure"
  bug_t        "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  bug_t        "This resolves project creation limitations for personal Google accounts."
  bug_e
  bug_section  "Key:"
  bug_tu       "   Magenta text refers to " "precise words you see on the web page."
  bug_tc       "   Cyan text is " "something you might copy from here."
  bug_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBGM_CLICK_MOD} + mouse click)"
  bug_e
  bug_section  "1. Confirm Payor Regime:"
  bug_tc       "   File: " "${ZRBGM_RBRP_FILE}"
  bug_tc       "   RBRP_PAYOR_PROJECT_ID: " "${RBRP_PAYOR_PROJECT_ID}"
  bug_t        "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  bug_e
  bug_t        "   First time setup? Set a timestamped project ID with:"
  bug_c        "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}")/' ${ZRBGM_RBRP_FILE}"
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

  bug_section  "Cloud Build Concurrent Build Capacity"
  bug_t        "Review your build capacity settings to ensure sufficient concurrent build execution."
  bug_t        "Recipe Bottle uses a private worker pool — quota is tracked per private pool host project."
  bug_e
  bug_t        "   Private pool machine types vs concurrency at 10-CPU quota:"
  bug_tc       "     e2-standard-2  " "(2 vCPU)   → 5 concurrent builds"
  bug_tc       "     e2-standard-8  " "(8 vCPU)   → 1 concurrent build"
  bug_tc       "     e2-standard-32 " "(32 vCPU)  → needs 32+ CPU quota"
  bug_e
  bug_section  "Key:"
  bug_tu       "   Magenta text refers to " "precise words you see on the web page."
  bug_tc       "   Cyan text is " "something you might copy from here."
  bug_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBGM_CLICK_MOD} + mouse click)"
  bug_e
  bug_section  "1. Current Regime Configuration:"
  bug_tc       "   RBRR_DEPOT_PROJECT_ID:          " "${RBRR_DEPOT_PROJECT_ID}"
  bug_tc       "   RBRR_GCP_REGION:                " "${RBRR_GCP_REGION}"
  bug_tc       "   RBRR_GCB_MACHINE_TYPE:          " "${RBRR_GCB_MACHINE_TYPE}"
  bug_tc       "   RBRR_GCB_POOL_STEM:             " "${RBRR_GCB_POOL_STEM}"
  bug_tc       "   RBRR_GCB_MIN_CONCURRENT_BUILDS: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS}"
  bug_e
  bug_t        "   The build preflight gate checks quota automatically before each build."
  bug_t        "   It computes: quota_vCPUs / machine_vCPUs >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  bug_e
  bug_section  "2. Check CPU Quota:"
  bug_t        "   Private pool quota is tracked under the depot project."
  bug_e
  bug_link     "   Go to: " "Quotas & System Limits (opens to depot project)" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  bug_tu       "   1. Verify project " "${RBRR_DEPOT_PROJECT_ID}" " is selected in the project picker"
  bug_tut      "   2. In the " "Enter property name or value" " filter bar, type:"
  bug_tc       "      " "concurrent_private"
  bug_tut      "   3. Select " "cloudbuild.googleapis.com/concurrent_private_pool_build_cpus" " from the autocomplete"
  bug_t        "   4. Multiple rows appear. Look for the row with Type column showing"
  bug_tut      "      " "Quota" " (not System limit) and your region in the Dimensions column"
  bug_t        "   5. Note the quota value and current usage percentage"
  bug_t        "      If usage is near 100% with one build, the machine type is too large for the quota"
  bug_e
  bug_section  "3. Request a Quota Increase (if needed):"
  bug_tut      "   On the " "Quota" " row identified above:"
  bug_tut      "   1. Click the three-dot menu " "⋮" " at the right end of the row"
  bug_tu       "   2. Select " "Edit quota"
  bug_tu       "   3. In the side panel, enter the new value in the " "New value" " field"
  bug_t        "      Recommended: 10 (allows 5 concurrent e2-standard-2 builds across both pools)"
  bug_tu       "   4. A " "Request description" " field appears. Enter:"
  bug_tc       "      " "Need parallel builds on private worker pool for CI/CD pipeline testing."
  bug_tu       "   5. Click " "Next"
  bug_t        "   6. Step 2/2 shows contact details (pre-filled from your Google account)"
  bug_tu       "   7. Click " "Submit request"
  bug_t        "      Increases are typically approved within minutes."
  bug_e
  bug_section  "4. Confirm Quota Increase:"
  bug_t        "   After approval, quotas can take up to 15 minutes to propagate."
  bug_link     "   Return to: " "Quotas & System Limits (opens to depot project)" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  bug_tut      "   Filter for " "concurrent_private" " again and verify the new value"
  bug_t        "   Verify: quota / vCPUs per machine type >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  bug_tc       "     Current target: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS} concurrent builds"
  bug_e

  buc_success "Cloud Build quota guide displayed"
}


# Dashboard status line — no sentinel (used pre-kindle by onboarding guide)
zrbgm_po_status() {
  local -r z_flag="${1:-}"
  local -r z_text="${2:-}"
  if test "${z_flag}" = "1"; then
    bug_ct " [*] " "${z_text}"
  else
    bug_t " [ ] ${z_text}"
  fi
}

# Extract a KEY=VALUE from a file; stdout empty if missing.  grep-only, no sourcing.
zrbgm_po_extract_capture() {
  local -r z_file="${1:-}"
  local -r z_key="${2:-}"
  test -n "${z_key}"  || return 1
  test -f "${z_file}" || return 1
  local z_line=""
  z_line=$(grep -m1 "^${z_key}=" "${z_file}") || return 1
  echo "${z_line#"${z_key}="}"
}

# Configuration review — displayed at level 0 (first thing a newcomer sees)
# Shows pre-selected RBRR defaults with orientation, plus BURC project structure.
# No sentinel: runs pre-kindle like the rest of the onboarding guide.
zrbgm_po_review_defaults() {
  local z_rbrr_file="${RBBC_rbrr_file}"

  # --- Pre-selected RBRR defaults ---
  bug_section "Pre-selected Defaults"
  bug_tc "  File: " "${z_rbrr_file}"
  bug_t  "  Edit this file if any defaults don't fit, then re-run this guide."
  bug_e
  bug_t  "  Infrastructure:"
  bug_tc "    RBRR_DNS_SERVER                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_DNS_SERVER)"
  bug_t  "      Resolver for bottle network connectivity checks (Google Public DNS)."
  bug_tc "    RBRR_GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
  bug_t  "      Compute Engine type for Cloud Build workers. Smallest reasonable option;"
  bug_t  "      balances cost and build speed."
  bug_tc "    RBRR_GCB_TIMEOUT               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_TIMEOUT)"
  bug_t  "      Maximum time for a single Cloud Build job (20 minutes)."
  bug_tc "    RBRR_GCB_MIN_CONCURRENT_BUILDS " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MIN_CONCURRENT_BUILDS)"
  bug_t  "      Preflight gate: require capacity for this many parallel builds."
  bug_t  "      With e2-standard-2 (2 vCPU) and default 10-CPU quota, 5 are possible."
  bug_e
  bug_t  "  Conventions:"
  bug_tc "    RBRR_VESSEL_DIR                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_VESSEL_DIR)"
  bug_t  "      Directory containing vessel specifications — what gets built into"
  bug_t  "      container images."
  bug_tc "    RBRR_GCP_REGION                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCP_REGION)"
  bug_t  "      GCP region for all infrastructure. Change if you need geographic"
  bug_t  "      proximity to a different region."
  bug_tc "    RBRR_SECRETS_DIR               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_SECRETS_DIR)"
  bug_t  "      Local directory for credential files, outside the repo for security."
  bug_t  "      Created automatically when credentials are first installed."
  bug_e

  # --- BURC project structure (read-only orientation) ---
  bug_section "Project Structure (BURC regime — rarely needs changing)"
  bug_tc "    BURC_TOOLS_DIR                 " "${BURC_TOOLS_DIR}"
  bug_t  "      Tool modules — the kit scripts that implement every operation."
  bug_tc "    BURC_TABTARGET_DIR             " "${BURC_TABTARGET_DIR}"
  bug_t  "      Launcher scripts. Tab-complete these in your terminal to run commands."
  bug_tc "    BURC_STATION_FILE              " "${BURC_STATION_FILE}"
  bug_t  "      Developer-local settings (log directory). Lives outside the repo."
  bug_tc "    BURC_TEMP_ROOT_DIR             " "${BURC_TEMP_ROOT_DIR}"
  bug_t  "      Temporary working files for each command run. Outside the repo."
  bug_tc "    BURC_OUTPUT_ROOT_DIR           " "${BURC_OUTPUT_ROOT_DIR}"
  bug_t  "      Persistent command output. Outside the repo."
  bug_tc "    BURC_MANAGED_KITS              " "${BURC_MANAGED_KITS}"
  bug_t  "      Kit modules managed by the vvx toolchain."
  bug_e
}

rbgm_onboarding() {
  # No zrbgm_sentinel — works pre-kindle (load-bearing: the guide's purpose
  # is to run before a valid regime exists; see docket for rationale)

  buc_doc_brief "Role-aware onboarding dashboard — probes credentials and shows per-role status"
  buc_doc_shown || return 0

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
  bug_section "Recipe Bottle Onboarding Dashboard"
  bug_e

  # ===================================================================
  # Role Inventory Display
  # ===================================================================
  bug_section "Role Inventory"
  bug_t "  Credential file presence IS the role declaration. No flags, no parameters."
  bug_t "  The filesystem is the configuration."
  bug_e

  zrbgm_po_status "${z_has_payor}"     "Payor       — OAuth credential: creates/funds GCP infrastructure"
  zrbgm_po_status "${z_has_governor}"  "Governor    — Service account: administers director/retriever credentials"
  zrbgm_po_status "${z_has_director}"  "Director    — Service account: submits builds, manages images"
  zrbgm_po_status "${z_has_retriever}" "Retriever   — Service account: pulls images for local bottles"
  bug_e

  # ===================================================================
  # Credential Guidance — for absent roles
  # ===================================================================
  if test "${z_has_payor}" = "0" || test "${z_has_governor}" = "0" || \
     test "${z_has_director}" = "0" || test "${z_has_retriever}" = "0"; then
    bug_section "Credential Guidance"

    if test "${z_has_payor}" = "0"; then
      bug_t "  Payor: OAuth credential (rbro-payor.env) not found."
      if test -n "${z_secrets_dir}"; then
        bug_tc "    Expected at: " "${z_secrets_dir}/rbro-payor.env"
      fi
      bug_t "    To become the payor, run Payor Establish then Payor Install:"
      buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
      buc_tabtarget "${RBZ_PAYOR_INSTALL}" "\${HOME}/Downloads/client_secret_*.json"
      bug_e
    fi

    if test "${z_has_governor}" = "0" && test "${z_has_director}" = "0" && \
       test "${z_has_retriever}" = "0" && test -n "${z_secrets_dir}"; then
      bug_t "  Service accounts (governor, director, retriever) authenticate via"
      bug_t "  RBRA credential files. Each is a shell-sourceable .env file placed in:"
      bug_tc "    " "${z_secrets_dir}/"
      bug_t "  Required permissions: 600. Expected filenames:"
      bug_t "    ${RBCC_role_governor}/${RBCC_rbra_file}   — admin for depot project"
      bug_t "    ${RBCC_role_director}/${RBCC_rbra_file}   — executes Cloud Build operations"
      bug_t "    ${RBCC_role_retriever}/${RBCC_rbra_file}  — pulls images for local bottles"
      bug_e
      bug_t "  If you received a credential file, place it at the path above."
      bug_t "  If you are the payor, create credentials via the payor track below."
      bug_e
    fi
  fi

  # ===================================================================
  # Phase 2: Per-role Status Tracks
  # ===================================================================

  # --- Payor Track ---
  if test "${z_has_payor}" = "1"; then
    bug_section "Payor Track"

    # Sub-probes: repo-level config facts (committed, not role-specific)
    local z_has_project=0
    if test -f "${RBBC_rbrp_file}"; then
      grep -q '^RBRP_PAYOR_PROJECT_ID=.\+' "${RBBC_rbrp_file}" && z_has_project=1
    fi
    local z_has_depot=0
    if test -f "${RBBC_rbrr_file}"; then
      grep -q '^RBRR_DEPOT_PROJECT_ID=.\+' "${RBBC_rbrr_file}" && z_has_depot=1
    fi

    zrbgm_po_status "${z_has_payor}"    "  OAuth installed"
    zrbgm_po_status "${z_has_project}"  "  Project configured"
    zrbgm_po_status "${z_has_depot}"    "  Depot created"
    zrbgm_po_status "${z_has_governor}" "  Governor reset"
    bug_e

    # Next step for payor
    if test "${z_has_project}" = "0"; then
      bug_t "  Next: Configure the payor project identity."
      bug_tc "    Edit: " "${RBBC_rbrp_file}"
      bug_t "    Set RBRP_PAYOR_PROJECT_ID, then run:"
      buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
    elif test "${z_has_depot}" = "0"; then
      bug_t "  Next: Create the GCP depot project."
      bug_t "  Review RBRR defaults before proceeding — one RBRR is tied to one depot."
      bug_e
      bug_tc "    RBRR_GCP_REGION                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCP_REGION)"
      bug_tc "    RBRR_GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
      bug_tc "    RBRR_VESSEL_DIR                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_VESSEL_DIR)"
      bug_tc "    RBRR_SECRETS_DIR               " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_SECRETS_DIR)"
      bug_e
      bug_t "  Run (~2 min):"
      buc_tabtarget "${RBZ_LEVY_DEPOT}" "<depot-name>"
    elif test "${z_has_governor}" = "0"; then
      bug_t "  Next: Mantle the governor service account."
      bug_t "  Run:"
      buc_tabtarget "${RBZ_MANTLE_GOVERNOR}"
    else
      bug_t "  Payor track complete. Governor, director, and retriever credentials"
      bug_t "  are issued from the governor track (below) or distributed to machines."
    fi
    bug_e
  fi

  # --- Director Track ---
  if test "${z_has_director}" = "1"; then
    bug_section "Director Track"

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
    bug_e

    # Next step for director
    if test "${z_has_reliquary}" = "0"; then
      bug_t "  Next: Inscribe reliquary — freeze tool images in GAR."
      bug_t "  Run (~6 min):"
      buc_tabtarget "${RBZ_INSCRIBE_RELIQUARY}"
      bug_t "  Then record the reliquary ID in the busybox vessel:"
      bug_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}/rbrv.env"
      bug_tc "    Set RBRV_RELIQUARY=" "<reliquary-id>"
    elif test "${z_has_busybox_built}" = "0"; then
      bug_t "  Next: Airgap build — busybox on the air-gapped pool."
      bug_t "  1. Enshrine busybox base image (~2 min):"
      buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}"
      bug_t "  2. Conjure busybox (~8 min):"
      buc_tabtarget "${RBZ_ORDAIN_CONSECRATION}" "${z_vessel_dir:-rbev-vessels}/${z_busybox_sigil}"
      bug_t "  3. Vouch (verify SLSA provenance):"
      buc_tabtarget "${RBZ_VOUCH_CONSECRATIONS}"
    elif test "${z_has_bottle_built}" = "0"; then
      bug_t "  Next: Ifrit bottle — Debian bookworm-slim on tether pool."
      bug_t "  1. Record reliquary in bottle vessel:"
      bug_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}/rbrv.env"
      bug_tc "    Set RBRV_RELIQUARY=" "${z_busybox_reliquary:-<reliquary-id>}"
      bug_t "  2. Enshrine bottle base image (~2 min):"
      buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}"
      bug_t "  3. Conjure ifrit bottle (~8 min):"
      buc_tabtarget "${RBZ_ORDAIN_CONSECRATION}" "${z_vessel_dir:-rbev-vessels}/${z_bottle_sigil}"
      bug_t "  4. Vouch:"
      buc_tabtarget "${RBZ_VOUCH_CONSECRATIONS}"
    elif test "${z_has_sentry_built}" = "0"; then
      bug_t "  Next: Sentry — Debian bookworm-slim on tether pool."
      bug_t "  1. Record reliquary in sentry vessel:"
      bug_tc "    Edit: " "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}/rbrv.env"
      bug_tc "    Set RBRV_RELIQUARY=" "${z_busybox_reliquary:-<reliquary-id>}"
      if test -n "${z_sentry_anchor}" && test -n "${z_bottle_anchor}" && \
         test "${z_sentry_anchor}" = "${z_bottle_anchor}"; then
        bug_t "  2. Enshrine: SKIP — same base image already enshrined (shared enshrine namespace)"
      else
        bug_t "  2. Enshrine sentry base image (~2 min):"
        buc_tabtarget "${RBZ_ENSHRINE_VESSEL}" "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}"
      fi
      bug_t "  3. Conjure sentry (~5 min):"
      buc_tabtarget "${RBZ_ORDAIN_CONSECRATION}" "${z_vessel_dir:-rbev-vessels}/${z_sentry_sigil}"
      bug_t "  4. Vouch:"
      buc_tabtarget "${RBZ_VOUCH_CONSECRATIONS}"
    else
      bug_t "  Director track complete. All vessels built and vouched."
    fi
    bug_e
  fi

  # --- Retriever Track ---
  if test "${z_has_retriever}" = "1"; then
    bug_section "Retriever Track"

    # Sub-probe: nameplate consecrations populated
    local z_has_bottle_consecration=0
    local z_has_sentry_consecration=0
    if test -f "${ZRBGM_ONBOARDING_NAMEPLATE}"; then
      source "${ZRBGM_ONBOARDING_NAMEPLATE}"
      test -n "${RBRN_BOTTLE_CONSECRATION:-}" && z_has_bottle_consecration=1
      test -n "${RBRN_SENTRY_CONSECRATION:-}" && z_has_sentry_consecration=1
    fi

    # Sub-probe: vouch images present in local runtime
    local z_has_bottle_summoned=0
    local z_has_sentry_summoned=0
    if test "${z_has_bottle_consecration}" = "1" && test "${z_has_sentry_consecration}" = "1" && \
       test -f "${RBBC_rbrr_file}"; then
      source "${RBBC_rbrr_file}"
      local z_gar_host="${RBRR_GCP_REGION}-docker.pkg.dev"
      local z_bottle_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_CONSECRATION}-vouch"
      local z_sentry_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_CONSECRATION}-vouch"
      "${RBRN_RUNTIME}" image inspect "${z_bottle_vouch_ref}" >/dev/null 2>&1 && z_has_bottle_summoned=1
      "${RBRN_RUNTIME}" image inspect "${z_sentry_vouch_ref}" >/dev/null 2>&1 && z_has_sentry_summoned=1
    fi

    zrbgm_po_status "${z_has_retriever}"           "  Credentials present"
    zrbgm_po_status "${z_has_bottle_consecration}"  "  Bottle consecration recorded"
    zrbgm_po_status "${z_has_sentry_consecration}"  "  Sentry consecration recorded"
    zrbgm_po_status "${z_has_bottle_summoned}"      "  Bottle image summoned"
    zrbgm_po_status "${z_has_sentry_summoned}"      "  Sentry image summoned"
    bug_e

    # Next step for retriever
    if test "${z_has_bottle_consecration}" = "0"; then
      bug_t "  Next: Record bottle consecration in the tadmor nameplate."
      bug_t "  1. Edit the nameplate:"
      bug_tc "    " "${ZRBGM_ONBOARDING_NAMEPLATE}"
      bug_t "    Set (substitute your actual consecration value):"
      bug_tc "    RBRN_BOTTLE_CONSECRATION=" "<consecration>"
      bug_t "  2. Summon bottle:"
      buc_tabtarget "${RBZ_SUMMON_CONSECRATION}" "${RBRN_BOTTLE_VESSEL:-${z_bottle_sigil}} <consecration>"
    elif test "${z_has_sentry_consecration}" = "0"; then
      bug_t "  Next: Record sentry consecration in the tadmor nameplate."
      bug_t "  1. Edit the nameplate:"
      bug_tc "    " "${ZRBGM_ONBOARDING_NAMEPLATE}"
      bug_t "    Set (substitute your actual consecration value):"
      bug_tc "    RBRN_SENTRY_CONSECRATION=" "<consecration>"
      bug_t "  2. Summon sentry:"
      buc_tabtarget "${RBZ_SUMMON_CONSECRATION}" "${RBRN_SENTRY_VESSEL:-${z_sentry_sigil}} <consecration>"
    elif test "${z_has_bottle_summoned}" = "0" || test "${z_has_sentry_summoned}" = "0"; then
      bug_t "  Next: Summon missing images locally."
      if test "${z_has_bottle_summoned}" = "0"; then
        bug_t "  Summon bottle:"
        buc_tabtarget "${RBZ_SUMMON_CONSECRATION}" "${RBRN_BOTTLE_VESSEL:-${z_bottle_sigil}} ${RBRN_BOTTLE_CONSECRATION:-<consecration>}"
      fi
      if test "${z_has_sentry_summoned}" = "0"; then
        bug_t "  Summon sentry:"
        buc_tabtarget "${RBZ_SUMMON_CONSECRATION}" "${RBRN_SENTRY_VESSEL:-${z_sentry_sigil}} ${RBRN_SENTRY_CONSECRATION:-<consecration>}"
      fi
    else
      bug_t "  Retriever track complete. Run the tadmor security tests:"
      bug_tc "    " "tt/rbw-tf.TestFixture.tadmor-security.sh"
      bug_e
      bug_t "  Or start a bottle:"
      buc_tabtarget "${RBZ_CRUCIBLE_CHARGE}" "tadmor"
    fi
    bug_e
  fi

  # ===================================================================
  # No roles detected — first-time user guidance
  # ===================================================================
  if test "${z_has_payor}" = "0" && test "${z_has_governor}" = "0" && \
     test "${z_has_director}" = "0" && test "${z_has_retriever}" = "0"; then
    bug_section "Next Step"
    bug_t "  No roles detected on this machine. Two paths forward:"
    bug_e
    bug_t "  A. Full setup (you are the payor):"
    buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
    bug_e
    bug_t "  B. Retriever-only (you received credential files):"
    bug_t "     Place ${RBCC_role_retriever}/${RBCC_rbra_file} in RBRR_SECRETS_DIR and re-run this guide."
  fi

  buc_success "Onboarding dashboard displayed"
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
