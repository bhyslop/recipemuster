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
  if [ -z "${NO_COLOR:-}" ] && [ "${BURE_COLOR:-0}" = "1" ]; then
    z_use_color=1
  fi

  if [ "$z_use_color" = "1" ]; then
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
  bug_t        "Each build consumes vCPUs according to the Compute Engine machine type on the pool."
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
  bug_tc       "   RBRR_GCB_WORKER_POOL:           " "${RBRR_GCB_WORKER_POOL}"
  bug_tc       "   RBRR_GCB_MIN_CONCURRENT_BUILDS: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS}"
  bug_e
  bug_t        "   The build preflight gate checks quota automatically before each build."
  bug_t        "   It computes: quota_vCPUs / machine_vCPUs >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  bug_e
  bug_section  "2. Check CPU Quota:"
  bug_t        "   Private pool quota is tracked under the depot project with the metric:"
  bug_tc       "      " "concurrent_private_pool_build_cpus"
  bug_e
  bug_link     "   Go to: " "Quotas & System Limits" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  bug_tu       "   1. Verify project " "${RBRR_DEPOT_PROJECT_ID}" " is selected in the project picker"
  bug_t        "   2. In the filter bar, enter:"
  bug_tc       "      " "cloudbuild.googleapis.com"
  bug_tut      "   3. Locate " "concurrent_private_pool_build_cpus" " for your region"
  bug_t        "   4. Note the quota value and current usage percentage"
  bug_t        "      If usage is near 100% with one build, the machine type is too large for the quota"
  bug_e
  bug_section  "3. Adjust Machine Type for Capacity:"
  bug_t        "   Machine type is the primary control for concurrent build capacity."
  bug_t        "   Use Compute Engine machine type names in RBRR_GCB_MACHINE_TYPE."
  bug_tc       "     Set RBRR_GCB_MACHINE_TYPE=" "e2-standard-2"
  bug_t        "     This uses 2 vCPUs per build, allowing 5 concurrent in a 10-CPU quota."
  bug_t        "   Then update the pool via workerPools.patch API to apply the change."
  bug_e
  bug_t        "   If machine type adjustment cannot meet your capacity needs, a CPU quota increase"
  bug_t        "   can be requested via Console Edit Quotas. This is a last resort and rarely necessary."
  bug_e
  bug_section  "4. Confirm Quota Headroom:"
  bug_link     "   Return to: " "Quotas & System Limits" "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}"
  bug_t        "   Filter for cloudbuild.googleapis.com"
  bug_t        "   Verify: quota / vCPUs per machine type >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  bug_tc       "     Current target: " "${RBRR_GCB_MIN_CONCURRENT_BUILDS} concurrent builds"
  bug_e

  buc_success "Cloud Build quota guide displayed"
}

rbgm_gitlab_setup() {
  zrbgm_sentinel

  buc_doc_brief "Display GitLab rubric repo setup guide for CB v2 connections"
  buc_doc_shown || return 0

  bug_section  "GitLab Rubric Repo Setup Guide"
  bug_t        "Recipe Bottle uses GitLab (not GitHub) for the rubric repo connection."
  bug_t        "This is required before running depot_create."
  bug_e
  bug_section  "Why GitLab (not GitHub)?"
  bug_t        "  GitHub classic PATs grant 'repo' scope across ALL repositories the token"
  bug_t        "  owner can access. Fine-grained PATs are rejected by Cloud Build v2 connections"
  bug_t        "  (Google Issue Tracker #343223837). Machine accounts are an alternative but add"
  bug_t        "  organizational overhead."
  bug_e
  bug_t        "  GitLab project access tokens are inherently repository-scoped — they cannot"
  bug_t        "  access resources outside the associated project. One token, one repo, minimal scope."
  bug_e
  bug_section  "Key:"
  bug_tu       "   Magenta text refers to " "precise words you see on the web page."
  bug_tc       "   Cyan text is " "something you might copy from here."
  bug_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBGM_CLICK_MOD} + mouse click)"
  bug_e
  bug_section  "1. Create GitLab Account (if needed):"
  bug_link     "   Go to: " "GitLab Sign Up" "https://gitlab.com/users/sign_up"
  bug_t        "   Create a free account if you don't already have one."
  bug_e
  bug_section  "2. Create a GitLab Project for the Rubric Repo:"
  bug_link     "   Go to: " "GitLab New Project" "https://gitlab.com/projects/new#blank_project"
  bug_tu       "   1. Select " "Create blank project"
  bug_t        "   2. Configure:"
  bug_tc       "      - Project name: " "rb-rubric"
  bug_t        "      - Project slug: auto-fills (leave as-is)"
  bug_tut      "      - Project deployment target: " "skip" " (leave as 'Select the deployment target')"
  bug_tut      "      - Visibility Level: " "Private" " (rubric repo has no reason to be public)"
  bug_tut      "      - Check " "Initialize repository with a README" " (CB v2 needs a non-empty repo)"
  bug_t        "      - SAST / Secret Detection: leave unchecked (unnecessary for generated build files)"
  bug_tu       "   3. Click " "Create project"
  bug_e
  bug_section  "3. Set RBRR_RUBRIC_REPO_URL:"
  bug_t        "   After creation, copy the HTTPS clone URL from the project page."
  bug_tc       "   Example: " "RBRR_RUBRIC_REPO_URL=https://gitlab.com/yourname/rb-rubric.git"
  bug_tc       "   Edit " "${RBBC_rbrr_file}" " and set this value."
  bug_e
  bug_section  "4. Create a Project Access Token:"
  local z_tokens_url=""
  z_tokens_url=$(zrbgu_gitlab_tokens_url_capture 2>/dev/null) || z_tokens_url=""
  if test -n "${z_tokens_url}"; then
    bug_link     "   Go to: " "Project Access Tokens" "${z_tokens_url}"
  else
    bug_t        "   From your rubric repo project page:"
    bug_tu       "   Left sidebar: " "Settings" " → "
    bug_tu       "                 " "Access tokens"
  fi
  bug_tu       "   Click " "Add new token"
  bug_t        "   Configure:"
  bug_tc       "      - Token name: " "rb-depot"
  bug_t        "      - Token description: (optional, skip)"
  bug_t        "      - Expiration date: default is 30 days (max 1 year); leave default or extend"
  bug_tutu     "      - Select a role: change dropdown from " "Guest" " to " "Maintainer"
  bug_tutu     "      - Select scopes: check both " "api" " and " "read_api"
  bug_tu       "   Click " "Create project access token"
  bug_tW       "   4. " "CRITICAL: Copy the token immediately — it won't be shown again"
  bug_e
  bug_section  "5. Run Depot Create:"
  bug_t        "   Run depot_create — it will prompt you to paste the token:"
  buc_tabtarget "${RBZ_CREATE_DEPOT}" "<depot-name>"
  bug_t        "   When prompted, paste the token and press Enter."
  bug_tW       "   " "Do NOT put the token in a command line or pipeline — stdin keeps it out of shell history."

  buc_success "GitLab rubric repo setup guide displayed"
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
  bug_tc "    DNS_SERVER                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_DNS_SERVER)"
  bug_t  "      Resolver for bottle network connectivity checks (Google Public DNS)."
  bug_tc "    GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
  bug_t  "      Compute Engine type for Cloud Build workers. Smallest reasonable option;"
  bug_t  "      balances cost and build speed."
  bug_tc "    GCB_TIMEOUT               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_TIMEOUT)"
  bug_t  "      Maximum time for a single Cloud Build job (20 minutes)."
  bug_tc "    GCB_MIN_CONCURRENT_BUILDS " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCB_MIN_CONCURRENT_BUILDS)"
  bug_t  "      Preflight gate: require capacity for this many parallel builds."
  bug_t  "      With e2-standard-2 (2 vCPU) and default 10-CPU quota, 5 are possible."
  bug_e
  bug_t  "  Conventions:"
  bug_tc "    VESSEL_DIR                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_VESSEL_DIR)"
  bug_t  "      Directory containing vessel specifications — what gets built into"
  bug_t  "      container images."
  bug_tc "    GCP_REGION                " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_GCP_REGION)"
  bug_t  "      GCP region for all infrastructure. Change if you need geographic"
  bug_t  "      proximity to a different region."
  bug_tc "    SECRETS_DIR               " "$(zrbgm_po_extract_capture "${z_rbrr_file}" RBRR_SECRETS_DIR)"
  bug_t  "      Local directory for credential files, outside the repo for security."
  bug_t  "      Created automatically when credentials are first installed."
  bug_e

  # --- BURC project structure (read-only orientation) ---
  bug_section "Project Structure (BURC regime — rarely needs changing)"
  bug_tc "    TOOLS_DIR                 " "${BURC_TOOLS_DIR}"
  bug_t  "      Tool modules — the kit scripts that implement every operation."
  bug_tc "    TABTARGET_DIR             " "${BURC_TABTARGET_DIR}"
  bug_t  "      Launcher scripts. Tab-complete these in your terminal to run commands."
  bug_tc "    STATION_FILE              " "${BURC_STATION_FILE}"
  bug_t  "      Developer-local settings (log directory). Lives outside the repo."
  bug_tc "    TEMP_ROOT_DIR             " "${BURC_TEMP_ROOT_DIR}"
  bug_t  "      Temporary working files for each command run. Outside the repo."
  bug_tc "    OUTPUT_ROOT_DIR           " "${BURC_OUTPUT_ROOT_DIR}"
  bug_t  "      Persistent command output. Outside the repo."
  bug_tc "    MANAGED_KITS              " "${BURC_MANAGED_KITS}"
  bug_t  "      Kit modules managed by the vvx toolchain."
  bug_e
}

rbgm_payor_onboarding() {
  # No zrbgm_sentinel — works pre-kindle (load-bearing: the guide's purpose
  # is to run before a valid regime exists; see docket for rationale)

  buc_doc_brief "Adaptive onboarding guide — reads current state and shows next steps"
  buc_doc_shown || return 0

  # --- Compute level (strict sequential: first unmet probe stops advancement) ---
  local z_level=0
  local z_secrets_dir=""
  local z_gar_host=""
  local z_sentry_vouch_ref=""
  local z_bottle_vouch_ref=""

  while true; do
    # Level 1: Payor project configured in rbrp.env
    test -f "${RBBC_rbrp_file}" || break
    grep -q '^RBRP_PAYOR_PROJECT_ID=.\+' "${RBBC_rbrp_file}" || break
    z_level=1

    # Level 2: OAuth credentials installed (requires secrets dir from rbrr)
    test -f "${RBBC_rbrr_file}" || break
    z_secrets_dir=$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" "RBRR_SECRETS_DIR") || z_secrets_dir=""
    test -n "${z_secrets_dir}" || break
    test -f "${z_secrets_dir}/rbro-payor.env" || break
    z_level=2

    # Level 3: GitLab rubric repo URL configured
    grep -q '^RBRR_RUBRIC_REPO_URL=.\+' "${RBBC_rbrr_file}" || break
    z_level=3

    # Level 4: Depot project created
    grep -q '^RBRR_DEPOT_PROJECT_ID=.\+' "${RBBC_rbrr_file}" || break
    z_level=4

    # Level 5: Governor service account
    test -f "${z_secrets_dir}/rbra-governor.env" || break
    z_level=5

    # Level 6: Director service account
    test -f "${z_secrets_dir}/rbra-director.env" || break
    z_level=6

    # Level 7: Retriever service account
    test -f "${z_secrets_dir}/rbra-retriever.env" || break
    z_level=7

    # Level 8: nsproto consecrations present (conjure completed)
    test -f "${RBBC_dot_dir}/rbrn_nsproto.env" || break
    source "${RBBC_dot_dir}/rbrn_nsproto.env"
    test -n "${RBRN_SENTRY_CONSECRATION:-}" || break
    test -n "${RBRN_BOTTLE_CONSECRATION:-}" || break
    z_level=8

    # Level 9: nsproto vouch images present in local runtime (vouch & summon completed)
    source "${RBBC_rbrr_file}"
    z_gar_host="${RBRR_GCP_REGION}-docker.pkg.dev"
    z_sentry_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_SENTRY_VESSEL}:${RBRN_SENTRY_CONSECRATION}-vouch"
    z_bottle_vouch_ref="${z_gar_host}/${RBRR_DEPOT_PROJECT_ID}/${RBRR_GAR_REPOSITORY}/${RBRN_BOTTLE_VESSEL}:${RBRN_BOTTLE_CONSECRATION}-vouch"
    "${RBRN_RUNTIME}" image inspect "${z_sentry_vouch_ref}" >/dev/null 2>&1 || break
    "${RBRN_RUNTIME}" image inspect "${z_bottle_vouch_ref}" >/dev/null 2>&1 || break
    z_level=9

    break
  done

  # --- Frontmatter (level-appropriate orientation for newcomers) ---
  case "${z_level}" in
    0)
      bug_section "Recipe Bottle Onboarding — Configuration Review"
      bug_t "Welcome to Recipe Bottle. This guide walks you through provisioning"
      bug_t "Google Cloud infrastructure from scratch. Before starting, review"
      bug_t "the configuration defaults below."
      bug_e
      zrbgm_po_review_defaults
      ;;
    1)
      bug_section "Recipe Bottle Onboarding — Payor Established"
      bug_t "Your GCP payor project exists. Next you will install OAuth"
      bug_t "credentials so that CLI tools can act on your behalf."
      ;;
    2)
      bug_section "Recipe Bottle Onboarding — Payor Credentialed"
      bug_t "Payor RBRA credentials are emplaced. Next you will connect a GitLab"
      bug_t "rubric repo where Cloud Build fetches build definitions."
      ;;
    3)
      bug_section "Recipe Bottle Onboarding — Source Connected"
      bug_t "GitLab rubric repo is configured. Next you will create the GCP"
      bug_t "depot project that hosts build infrastructure, artifact registry,"
      bug_t "and secrets."
      ;;
    4)
      bug_section "Recipe Bottle Onboarding — Depot Created"
      bug_t "Depot project exists. Three service accounts are needed:"
      bug_t "governor (admin), director (builds), and retriever (image pulls)."
      ;;
    5)
      bug_section "Recipe Bottle Onboarding — Governor Provisioned"
      bug_t "Governor admin account is ready. Next: create the director"
      bug_t "service account that executes Cloud Build operations."
      ;;
    6)
      bug_section "Recipe Bottle Onboarding — Director Provisioned"
      bug_t "Director build account is ready. Next: create the retriever"
      bug_t "service account for pulling container images to local bottles."
      ;;
    7)
      bug_section "Recipe Bottle Onboarding — Service Accounts Ready"
      bug_t "All service accounts are provisioned. Next: build the nsproto"
      bug_t "vessel images with conjure."
      ;;
    8)
      bug_section "Recipe Bottle Onboarding — Vessel Images Built"
      bug_t "Nsproto vessel images are built. Next: verify with batch vouch,"
      bug_t "then summon both vessels to pull images locally."
      ;;
    9)
      bug_section "Recipe Bottle Onboarding — Setup Complete"
      bug_t "Infrastructure is fully provisioned and vessel images are verified."
      bug_t "You can now start bottles from your built vessel images."
      ;;
  esac
  bug_e

  # --- Dashboard ---
  local z_flag=0
  z_flag=0; test "${z_level}" -ge 1 && z_flag=1
  zrbgm_po_status "${z_flag}" "1. Payor Establish     — GCP project + OAuth consent screen"
  z_flag=0; test "${z_level}" -ge 2 && z_flag=1
  zrbgm_po_status "${z_flag}" "2. Payor Install       — RBRA credential emplacement"
  z_flag=0; test "${z_level}" -ge 3 && z_flag=1
  zrbgm_po_status "${z_flag}" "3. GitLab Setup        — Rubric repo + access token"
  z_flag=0; test "${z_level}" -ge 4 && z_flag=1
  zrbgm_po_status "${z_flag}" "4. Depot Create        — GCP depot project"
  z_flag=0; test "${z_level}" -ge 5 && z_flag=1
  zrbgm_po_status "${z_flag}" "5. Governor Reset      — Admin service account"
  z_flag=0; test "${z_level}" -ge 6 && z_flag=1
  zrbgm_po_status "${z_flag}" "6. Director Create     — Build service account"
  z_flag=0; test "${z_level}" -ge 7 && z_flag=1
  zrbgm_po_status "${z_flag}" "7. Retriever Create    — Image pull service account"
  z_flag=0; test "${z_level}" -ge 8 && z_flag=1
  zrbgm_po_status "${z_flag}" "8. Conjure             — Build nsproto vessel images"
  z_flag=0; test "${z_level}" -ge 9 && z_flag=1
  zrbgm_po_status "${z_flag}" "9. Vouch & Summon      — Verify and pull vessel images"
  bug_e

  # --- Next step guidance ---
  case "${z_level}" in
    0)
      bug_section "Next: Payor Establish"
      bug_t "  If the defaults above look right, proceed to Payor Establish."
      bug_tc "  To adjust defaults first, edit " "${RBBC_rbrr_file}"
      bug_t "  and re-run this guide. Nothing is committed until the next step."
      bug_e
      bug_t "  Create a GCP project and configure OAuth consent screen."
      bug_t "  This is the billing anchor for all Recipe Bottle infrastructure."
      bug_e
      bug_t "  Run the guided procedure:"
      buc_tabtarget "${RBZ_PAYOR_ESTABLISH}"
      ;;
    1)
      bug_section "Next: Payor Install"
      bug_t "  Install OAuth credentials from the JSON file downloaded during Payor Establish."
      bug_e
      bug_t "  Run:"
      buc_tabtarget "${RBZ_PAYOR_INSTALL}" "~/Downloads/client_secret_*.json"
      ;;
    2)
      bug_section "Next: GitLab Setup"
      bug_t "  Cloud Build needs build instructions, but should never see your main repo."
      bug_t "  The rubric repo is a separate, minimal repository that serves as the"
      bug_t "  security boundary between your project and Google."
      bug_e
      bug_t "  You define vessels in your main repo. When you inscribe, Recipe Bottle"
      bug_t "  translates your vessel definitions into build instructions that Cloud Build"
      bug_t "  understands, and pushes them to the rubric repo automatically. You never"
      bug_t "  edit the rubric repo directly."
      bug_e
      bug_t "  GitLab is required: its project access tokens are repository-scoped,"
      bug_t "  which Cloud Build's v2 connection API needs."
      bug_e
      bug_t "  Run the guided procedure:"
      buc_tabtarget "${RBZ_GITLAB_SETUP}"
      ;;
    3)
      bug_section "Next: Depot Create"
      bug_t "  Creating a depot binds your RBRR configuration to real cloud resources."
      bug_t "  Review these values before proceeding — one RBRR is tied to one depot."
      bug_e
      bug_tc "    GCP_REGION                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCP_REGION)"
      bug_tc "    GCB_MACHINE_TYPE          " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCB_MACHINE_TYPE)"
      bug_tc "    GCB_TIMEOUT               " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCB_TIMEOUT)"
      bug_tc "    GCB_MIN_CONCURRENT_BUILDS " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_GCB_MIN_CONCURRENT_BUILDS)"
      bug_tc "    VESSEL_DIR                " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_VESSEL_DIR)"
      bug_tc "    SECRETS_DIR               " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_SECRETS_DIR)"
      bug_tc "    RUBRIC_REPO_URL           " "$(zrbgm_po_extract_capture "${RBBC_rbrr_file}" RBRR_RUBRIC_REPO_URL)"
      bug_e
      bug_tc "  To adjust, edit " "${RBBC_rbrr_file}"
      bug_t "  and re-run this guide."
      bug_e
      bug_t "  Create the GCP depot project that hosts all build infrastructure."
      bug_e
      bug_t "  Run:"
      buc_tabtarget "${RBZ_CREATE_DEPOT}" "<depot-name>"
      ;;
    4)
      bug_section "Next: Governor Reset"
      bug_t "  Create the governor service account (admin for depot project)."
      bug_e
      bug_t "  Run:"
      buc_tabtarget "${RBZ_GOVERNOR_RESET}"
      ;;
    5)
      bug_section "Next: Director Create"
      bug_t "  Create the director service account (executes Cloud Build operations)."
      bug_t "  The instance name labels this director — use a short identifier."
      bug_e
      bug_t "  Run:"
      buc_tabtarget "${RBZ_CREATE_DIRECTOR}" "<instance-name>"
      ;;
    6)
      bug_section "Next: Retriever Create"
      bug_t "  Create the retriever service account (pulls images for local bottles)."
      bug_t "  Use the same instance name as your director."
      bug_e
      bug_t "  Run:"
      buc_tabtarget "${RBZ_CREATE_RETRIEVER}" "<instance-name>"
      ;;
    7)
      bug_section "Next: Conjure"
      bug_t "  Build nsproto vessel images. Conjure each vessel separately:"
      bug_e
      bug_t "  1. Sentry vessel:"
      buc_tabtarget "${RBZ_CONJURE_ARK}" "${RBRN_SENTRY_VESSEL}"
      bug_t "  2. Bottle vessel:"
      buc_tabtarget "${RBZ_CONJURE_ARK}" "${RBRN_BOTTLE_VESSEL}"
      bug_e
      bug_t "  After each conjure completes, update the consecration in"
      bug_tc "  " "${RBBC_dot_dir}/rbrn_nsproto.env"
      ;;
    8)
      bug_section "Next: Vouch & Summon"
      bug_t "  Verify SLSA provenance on built images, then pull locally."
      bug_e
      bug_t "  1. Batch vouch (verifies all pending consecrations):"
      buc_tabtarget "${RBZ_VOUCH_ARK}"
      bug_t "  2. Summon sentry vessel:"
      buc_tabtarget "${RBZ_SUMMON_ARK}" "${RBRN_SENTRY_VESSEL} ${RBRN_SENTRY_CONSECRATION}"
      bug_t "  3. Summon bottle vessel:"
      buc_tabtarget "${RBZ_SUMMON_ARK}" "${RBRN_BOTTLE_VESSEL} ${RBRN_BOTTLE_CONSECRATION}"
      ;;
    9)
      bug_section "Next: Start a Bottle"
      bug_t "  Launch a bottle from your built vessel images:"
      bug_e
      buc_tabtarget "${RBZ_BOTTLE_START}" "<vessel-name>"
      ;;
  esac

  buc_success "Onboarding guide displayed"
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
