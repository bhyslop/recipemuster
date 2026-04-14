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
# Recipe Bottle Handbook - Payor-only Ceremonies
#
# Credential-gated manual procedures only the GCP project owner (payor)
# can perform: OAuth consent screen establishment, OAuth token refresh,
# Cloud Build quota review. Full regime + OAuth + IAM dependency stack.

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBHP_SOURCED:-}" || buc_die "Module rbhp multiply sourced - check sourcing hierarchy"
ZRBHP_SOURCED=1

######################################################################
# Internal Functions (zrbhp_*)

zrbhp_kindle() {
  test -z "${ZRBHP_KINDLED:-}" || buc_die "Module rbhp already kindled"

  # Click modifier: Cmd on macOS, Ctrl elsewhere
  case "$(uname -s)" in
    Darwin) readonly ZRBHP_CLICK_MOD="Cmd" ;;
    *)      readonly ZRBHP_CLICK_MOD="Ctrl" ;;
  esac

  readonly ZRBHP_RBRP_FILE="${RBBC_rbrp_file}"
  readonly ZRBHP_RBRP_FILE_BASENAME="${ZRBHP_RBRP_FILE##*/}"

  readonly ZRBHP_KINDLED=1
}

zrbhp_sentinel() {
  test "${ZRBHP_KINDLED:-}" = "1" || buc_die "Module rbhp not kindled - call zrbhp_kindle first"
}

zrbhp_enforce() {
  zrbhp_sentinel
  test -n "${RBRR_DEPOT_PROJECT_ID:-}"     || buc_die "RBRR_DEPOT_PROJECT_ID is not set"
  test   "${#RBRR_DEPOT_PROJECT_ID}" -gt 0 || buc_die "RBRR_DEPOT_PROJECT_ID is empty"
  zrbgc_sentinel
}

######################################################################
# External Functions (rbhp_*)

rbhp_establish() {
  zrbhp_sentinel

  buc_doc_brief "Display the manual Payor Establishment procedure for OAuth authentication"
  buc_doc_shown || return 0

  buh_section  "Manual Payor OAuth Establishment Procedure"
  buh_t        "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  buh_t        "This resolves project creation limitations for personal Google accounts."
  buh_e
  buh_section  "Key:"
  buh_tu       "   Magenta text refers to " "precise words you see on the web page."
  buh_tc       "   Cyan text is " "something you might copy from here."
  buh_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBHP_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Confirm Payor Regime:"
  buh_tc       "   File: " "${ZRBHP_RBRP_FILE}"
  buh_tc       "   RBRP_PAYOR_PROJECT_ID: " "${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  buh_e
  buh_t        "   First time setup? Set a timestamped project ID with:"
  buh_c        "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}")/' ${ZRBHP_RBRP_FILE}"
  buh_e
  buh_section  "2. Check if Project Already Exists:"
  buh_t        "   Before creating a new project, verify the configured ID is not already in use:"
  buh_link     "   1. Check existing projects: " "Google Cloud Project List" "https://console.cloud.google.com/cloud-resource-manager"
  buh_tu       "   2. Look for a project with ID " "${RBRP_PAYOR_PROJECT_ID}"
  buh_t        "      - Hover over project IDs to verify the full ID matches your configured value"
  buh_tctu     "   3. If you " "find the project" " with matching ID, it already exists - edit " "${ZRBHP_RBRP_FILE_BASENAME}"
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
  buh_tu       "   3. Save the billing account ID to your " "${ZRBHP_RBRP_FILE}"
  buh_tctu     "      Record as: " "RBRP_BILLING_ACCOUNT_ID=" " # " "Value from Account ID column"
  buh_t        "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  buh_tu       "   5. Update " "${ZRBHP_RBRP_FILE}" " and re-display this procedure."
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

}

rbhp_refresh() {
  zrbhp_sentinel

  buc_doc_brief "Display the manual Payor OAuth credential installation/refresh procedure"
  buc_doc_shown || return 0

  buh_section  "Manual Payor OAuth Credential Installation/Refresh Procedure"
  buh_t        "Use this for initial credential setup after payor establishment or to refresh expired/compromised credentials."
  buh_t        "Testing mode refresh tokens expire after 6 months of non-use."
  buh_e
  buh_section  "When to use this procedure:"
  buh_t        "  - Initial setup after running rbhp_establish"
  buh_t        "  - Payor operations return 401/403 errors"
  buh_t        "  - OAuth client secret compromised"
  buh_t        "  - 6+ months since last Payor operation"
  buh_e
  buh_section  "1. Obtain OAuth Credentials:"
  buh_t        "   For initial setup:"
  buh_t        "      - Use JSON file downloaded during rbhp_establish"
  buh_t        "   For refresh/renewal:"
  buh_link     "      - Go to: " "Credentials for Payor Project" "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}"
  buh_tut      "      - Find existing " "${RBGC_PAYOR_APP_NAME}" " OAuth client"
  buh_t        "      - Click the OAuth client name to open details"
  buh_t        "      - To rotate secret if compromised:"
  buh_tu       "        a. Click " "+ Add secret"
  buh_t        "        b. Click the download icon next to the NEW secret"
  buh_tu       "           Browser downloads: " "client_secret_[id].apps.googleusercontent.com.json"
  buh_tut      "        c. Click " "Disable" " on the secret with the older creation date"
  buh_t        "        d. Click the trash icon to delete that disabled secret"
  buh_e
  buh_section  "2. Install/Refresh OAuth Credentials:"
  buh_t        "   Run the payor install command with the downloaded JSON:"
  buh_tc       "      " "rbgp_payor_install ~/Downloads/client_secret_*.json"
  buh_t        "   This will:"
  buh_t        "   - Guide you through OAuth authorization flow"
  buh_t        "   - Store secure credentials in RBRR_SECRETS_DIR/rbro.env"
  buh_t        "   - Update RBRP_OAUTH_CLIENT_ID in rbrp.env"
  buh_t        "   - Test the authentication"
  buh_t        "   - Initialize depot tracking"
  buh_t        "   - Reset the 6-month expiration timer"
  buh_e
  buh_section  "3. Verify Installation:"
  buh_t        "   Test with a simple operation:"
  buc_tabtarget "${RBZ_LIST_DEPOT}"
  buh_t        "   Should display current depots without authentication errors."
  buh_e
  buh_t        "Prevention: Run any Payor operation monthly to prevent expiration."

}

rbhp_quota_build() {
  zrbhp_sentinel

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
  buh_link     "   Clickable links look like " "EXAMPLE DOT COM" "https://example.com/" " (often, ${ZRBHP_CLICK_MOD} + mouse click)"
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

}

# eof
