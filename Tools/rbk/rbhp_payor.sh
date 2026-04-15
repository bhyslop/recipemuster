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

  local z_ui z_ui2 z_cmd z_href z_warn

  buh_section  "Manual Payor OAuth Establishment Procedure"
  buh_line     "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  buh_line     "This resolves project creation limitations for personal Google accounts."
  buh_e
  buh_section  "Key:"
  buyy_ui_yawp "precise words you see on the web page."; z_ui="${z_buym_yelp}"
  buh_line     "   Magenta text refers to ${z_ui}"
  buyy_cmd_yawp "something you might copy from here."; z_cmd="${z_buym_yelp}"
  buh_line     "   Cyan text is ${z_cmd}"
  buyy_href_yawp "https://example.com/" "EXAMPLE DOT COM"; z_href="${z_buym_yelp}"
  buh_line     "   Clickable links look like ${z_href} (often, ${ZRBHP_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Confirm Payor Regime:"
  buyy_cmd_yawp "${ZRBHP_RBRP_FILE}"; z_cmd="${z_buym_yelp}"
  buh_line     "   File: ${z_cmd}"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRP_PAYOR_PROJECT_ID: ${z_cmd}"
  buh_line     "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  buh_e
  buh_line     "   First time setup? Set a timestamped project ID with:"
  buh_code     "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}")/' ${ZRBHP_RBRP_FILE}"
  buh_e
  buh_section  "2. Check if Project Already Exists:"
  buh_line     "   Before creating a new project, verify the configured ID is not already in use:"
  buyy_href_yawp "https://console.cloud.google.com/cloud-resource-manager" "Google Cloud Project List"; z_href="${z_buym_yelp}"
  buh_line     "   1. Check existing projects: ${z_href}"
  buyy_ui_yawp "${RBRP_PAYOR_PROJECT_ID}"; z_ui="${z_buym_yelp}"
  buh_line     "   2. Look for a project with ID ${z_ui}"
  buh_line     "      - Hover over project IDs to verify the full ID matches your configured value"
  buyy_cmd_yawp "find the project"; z_cmd="${z_buym_yelp}"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE_BASENAME}"; z_ui="${z_buym_yelp}"
  buh_line     "   3. If you ${z_cmd} with matching ID, it already exists - edit ${z_ui}"
  buh_line     "      and re-run this procedure"
  buh_line     "   4. If you don't find it, proceed to step 3 to create it"
  buh_e
  buh_section  "3. Create Payor Project:"
  buyy_href_yawp "https://console.cloud.google.com/projectcreate" "Google Cloud Project Create"; z_href="${z_buym_yelp}"
  buh_line     "   1. Open browser to: ${z_href}"
  buh_line     "   2. Ensure signed in with intended Google account (check top-right avatar)"
  buh_line     "   3. Configure new project:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; z_cmd="${z_buym_yelp}"
  buh_line     "      - Project name: ${z_cmd}"
  buh_line     "      - Project ID: Google will auto-generate a value; click Edit to replace it with:"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; z_cmd="${z_buym_yelp}"
  buh_line     "        ${z_cmd}"
  buyy_ui_yawp "No organization"; z_ui="${z_buym_yelp}"
  buh_line     "      - Location: ${z_ui} (required for this guide; organization affiliation is advanced)"
  buyy_ui_yawp "CREATE"; z_ui="${z_buym_yelp}"
  buh_line     "   4. Click ${z_ui}"
  buyy_ui_yawp "Creating project..."; z_ui="${z_buym_yelp}"
  buh_line     "   5. Wait for ${z_ui} notification to complete"
  buh_e
  buh_section  "4. Verify Project Creation:"
  buh_line     "   Verify that your rbrp.env configuration matches the created project:"
  buyy_href_yawp "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}" "Google Cloud APIs Dashboard"; z_href="${z_buym_yelp}"
  buh_line     "   1. Test this link: ${z_href}"
  buh_line     "   2. If the page loads and shows your project, your configuration is correct"
  buyy_ui_yawp "You need additional access"; z_ui="${z_buym_yelp}"
  buh_line     "   3. If you see ${z_ui}, wait a few minutes and refresh the page"
  buyy_href_yawp "https://cloud.google.com/iam/docs/access-change-propagation" "Access Change Propagation"; z_href="${z_buym_yelp}"
  buh_line     "      GCP IAM changes are eventually consistent: ${z_href}"
  buh_e
  buh_section  "5. Configure Billing Account:"
  buyy_href_yawp "https://console.cloud.google.com/billing" "Google Cloud Billing"; z_href="${z_buym_yelp}"
  buh_line     "   1. Go to: ${z_href}"
  buh_line     "      If no billing accounts exist:"
  buyy_ui_yawp "CREATE ACCOUNT"; z_ui="${z_buym_yelp}"
  buh_line     "          a. Click ${z_ui}"
  buh_line     "          b. Configure payment method and submit"
  buyy_ui_yawp "Account ID"; z_ui="${z_buym_yelp}"
  buh_line     "          c. Copy new ${z_ui} from table"
  buh_line     "      else if single Open account exists:"
  buyy_ui_yawp "Account ID"; z_ui="${z_buym_yelp}"
  buh_line     "          a. Copy the ${z_ui} value"
  buh_line     "      else if multiple Open accounts exist:"
  buh_line     "          a. Choose account for Recipe Bottle funding"
  buyy_ui_yawp "Account ID"; z_ui="${z_buym_yelp}"
  buh_line     "          b. Copy chosen ${z_ui} value"
  buyy_href_yawp "https://console.cloud.google.com/billing/projects" "Google Cloud Billing Projects"; z_href="${z_buym_yelp}"
  buh_line     "   2. Go to: ${z_href}"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE}"; z_ui="${z_buym_yelp}"
  buh_line     "   3. Save the billing account ID to your ${z_ui}"
  buyy_cmd_yawp "RBRP_BILLING_ACCOUNT_ID="; z_cmd="${z_buym_yelp}"
  buyy_ui_yawp "Value from Account ID column"; z_ui="${z_buym_yelp}"
  buh_line     "      Record as: ${z_cmd} # ${z_ui}"
  buh_line     "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE}"; z_ui="${z_buym_yelp}"
  buh_line     "   5. Update ${z_ui} and re-display this procedure."
  buh_e
  buh_section  "6. Link Billing to Payor Project:"
  buh_line     "   Link your billing account to the newly created project."
  buyy_href_yawp "https://console.cloud.google.com/billing/manage" "Google Cloud Billing Account Management"; z_href="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href}"
  buh_line     "   1. The page loads to your default billing account."
  buyy_ui_yawp "Select a billing account"; z_ui="${z_buym_yelp}"
  buh_line     "   2. If you have multiple billing accounts, use the ${z_ui} dropdown at top"
  buh_line     "      - Choose the account matching your RBRP_BILLING_ACCOUNT_ID"
  buyy_ui_yawp "Projects linked to this billing account"; z_ui="${z_buym_yelp}"
  buh_line     "   3. Look for the section ${z_ui}"
  buh_line     "   4. Verify your payor project appears in the table:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; z_cmd="${z_buym_yelp}"
  buh_line     "      - Project name: ${z_cmd}"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; z_cmd="${z_buym_yelp}"
  buh_line     "      - Project ID: ${z_cmd}"
  buh_line     "   5. If project is NOT listed and billing needs to be enabled:"
  buyy_href_yawp "https://console.cloud.google.com/billing/linkedaccount?project=${RBRP_PAYOR_PROJECT_ID}" "Project Billing"; z_href="${z_buym_yelp}"
  buh_line     "      - Go to: ${z_href}"
  buyy_ui_yawp "Link a Billing Account"; z_ui="${z_buym_yelp}"
  buh_line     "      - Click ${z_ui}"
  buh_line     "      - Select your billing account and confirm"
  buh_e
  buh_section  "7. Enable Required APIs:"
  buyy_href_yawp "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}" "APIs & Services for your payor project"; z_href="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href}"
  buyy_ui_yawp "+ ENABLE APIS AND SERVICES"; z_ui="${z_buym_yelp}"
  buh_line     "   1. Click ${z_ui}"
  buh_line     "   2. Search for and enable these APIs:"
  buyy_cmd_yawp "Cloud Resource Manager API"; z_cmd="${z_buym_yelp}"
  buh_line     "      - ${z_cmd}"
  buyy_cmd_yawp "Cloud Billing API"; z_cmd="${z_buym_yelp}"
  buh_line     "      - ${z_cmd}"
  buyy_cmd_yawp "Service Usage API"; z_cmd="${z_buym_yelp}"
  buh_line     "      - ${z_cmd} (often enabled by default, look for green check)"
  buyy_cmd_yawp "IAM Service Account Credentials API"; z_cmd="${z_buym_yelp}"
  buh_line     "      - ${z_cmd}"
  buyy_cmd_yawp "Artifact Registry API"; z_cmd="${z_buym_yelp}"
  buh_line     "      - ${z_cmd}"
  buh_line     "   These enable programmatic depot management operations."
  buh_e
  buh_section  "8. Configure OAuth Consent Screen:"
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID}" "OAuth consent screen"; z_href="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href}"
  buyy_ui_yawp "Google Auth Platform not configured yet"; z_ui="${z_buym_yelp}"
  buh_line     "   1. The console displays ${z_ui}"
  buyy_ui_yawp "Get started"; z_ui="${z_buym_yelp}"
  buh_line     "   2. Click ${z_ui}"
  buh_line     "   3. Complete the Project Configuration wizard:"
  buh_line     "      Step 1 - App Information:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; z_cmd="${z_buym_yelp}"
  buh_line     "        - App name: ${z_cmd}"
  buh_line     "        - User support email: (your email)"
  buyy_ui_yawp "Next"; z_ui="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui}"
  buh_line     "      Step 2 - Audience:"
  buyy_ui_yawp "External"; z_ui="${z_buym_yelp}"
  buh_line     "        - Select ${z_ui}"
  buyy_ui_yawp "Next"; z_ui="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui}"
  buh_line     "      Step 3 - Contact Information:"
  buh_line     "        - Email addresses: (your email), press Enter"
  buyy_ui_yawp "Next"; z_ui="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui}"
  buh_line     "      Step 4 - Finish:"
  buyy_ui_yawp "I agree to the Google API Services: User Data Policy"; z_ui="${z_buym_yelp}"
  buh_line     "        - Check ${z_ui}"
  buyy_ui_yawp "Continue"; z_ui="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui}"
  buyy_ui_yawp "Create"; z_ui="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui}"
  buh_e
  buh_line     "   4. Add your email as a test user (avoids Google app verification process):"
  buyy_ui_yawp "Audience"; z_ui="${z_buym_yelp}"
  buh_line     "      1. Click ${z_ui} in left sidebar"
  buyy_ui_yawp "Test users"; z_ui="${z_buym_yelp}"
  buh_line     "      2. Scroll down to section ${z_ui}"
  buyy_ui_yawp "+ Add users"; z_ui="${z_buym_yelp}"
  buh_line     "      3. Click ${z_ui}"
  buyy_ui_yawp "Add Users"; z_ui="${z_buym_yelp}"
  buh_line     "      4. Right-side panel titled ${z_ui} slides in"
  buh_line     "      5. Enter your email address in the field"
  buyy_ui_yawp "Save"; z_ui="${z_buym_yelp}"
  buh_line     "      6. Click ${z_ui}"
  buyy_ui_yawp "1 user"; z_ui="${z_buym_yelp}"
  buh_line     "      7. Verify ${z_ui} appears in OAuth user cap"
  buh_e
  buh_section  "9. Create OAuth 2.0 Client ID:"
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}" "Credentials"; z_href="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href}"
  buyy_ui_yawp "+ Create credentials"; z_ui="${z_buym_yelp}"
  buh_line     "   1. From top bar, click ${z_ui}"
  buyy_ui_yawp "OAuth client ID"; z_ui="${z_buym_yelp}"
  buh_line     "   2. Select ${z_ui}"
  buyy_ui_yawp "Desktop app"; z_ui="${z_buym_yelp}"
  buh_line     "   3. Application type: ${z_ui}"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; z_cmd="${z_buym_yelp}"
  buh_line     "   4. Name: ${z_cmd}"
  buyy_ui_yawp "CREATE"; z_ui="${z_buym_yelp}"
  buh_line     "   5. Click ${z_ui}"
  buyy_ui_yawp "OAuth client created"; z_ui="${z_buym_yelp}"
  buh_line     "   6. Popup titled ${z_ui} displays client ID and secret"
  buyy_ui_yawp "Download JSON"; z_ui="${z_buym_yelp}"
  buh_line     "   7. Click ${z_ui}"
  buyy_ui_yawp "OK"; z_ui="${z_buym_yelp}"
  buyy_ui_yawp "client_secret_[id].apps.googleusercontent.com.json"; z_ui2="${z_buym_yelp}"
  buh_line     "   8. Click ${z_ui} ; browser downloads ${z_ui2}"
  buyy_warn_yawp "CRITICAL: Save securely - contains client secret"; z_warn="${z_buym_yelp}"
  buh_line     "      ${z_warn}"
  buh_e
  buh_section  "10. Install OAuth Credentials:"
  buh_line     "   Run:"
  buh_code     "   rbgp_payor_install ~/Downloads/payor-oauth.json"
  buh_line     "   This will guide you through OAuth authorization and complete the setup."

}

rbhp_refresh() {
  zrbhp_sentinel

  buc_doc_brief "Display the manual Payor OAuth credential installation/refresh procedure"
  buc_doc_shown || return 0

  local z_ui z_cmd z_href

  buh_section  "Manual Payor OAuth Credential Installation/Refresh Procedure"
  buh_line     "Use this for initial credential setup after payor establishment or to refresh expired/compromised credentials."
  buh_line     "Testing mode refresh tokens expire after 6 months of non-use."
  buh_e
  buh_section  "When to use this procedure:"
  buh_line     "  - Initial setup after running rbhp_establish"
  buh_line     "  - Payor operations return 401/403 errors"
  buh_line     "  - OAuth client secret compromised"
  buh_line     "  - 6+ months since last Payor operation"
  buh_e
  buh_section  "1. Obtain OAuth Credentials:"
  buh_line     "   For initial setup:"
  buh_line     "      - Use JSON file downloaded during rbhp_establish"
  buh_line     "   For refresh/renewal:"
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}" "Credentials for Payor Project"; z_href="${z_buym_yelp}"
  buh_line     "      - Go to: ${z_href}"
  buyy_ui_yawp "${RBGC_PAYOR_APP_NAME}"; z_ui="${z_buym_yelp}"
  buh_line     "      - Find existing ${z_ui} OAuth client"
  buh_line     "      - Click the OAuth client name to open details"
  buh_line     "      - To rotate secret if compromised:"
  buyy_ui_yawp "+ Add secret"; z_ui="${z_buym_yelp}"
  buh_line     "        a. Click ${z_ui}"
  buh_line     "        b. Click the download icon next to the NEW secret"
  buyy_ui_yawp "client_secret_[id].apps.googleusercontent.com.json"; z_ui="${z_buym_yelp}"
  buh_line     "           Browser downloads: ${z_ui}"
  buyy_ui_yawp "Disable"; z_ui="${z_buym_yelp}"
  buh_line     "        c. Click ${z_ui} on the secret with the older creation date"
  buh_line     "        d. Click the trash icon to delete that disabled secret"
  buh_e
  buh_section  "2. Install/Refresh OAuth Credentials:"
  buh_line     "   Run the payor install command with the downloaded JSON:"
  buyy_cmd_yawp "rbgp_payor_install ~/Downloads/client_secret_*.json"; z_cmd="${z_buym_yelp}"
  buh_line     "      ${z_cmd}"
  buh_line     "   This will:"
  buh_line     "   - Guide you through OAuth authorization flow"
  buh_line     "   - Store secure credentials in RBRR_SECRETS_DIR/rbro.env"
  buh_line     "   - Update RBRP_OAUTH_CLIENT_ID in rbrp.env"
  buh_line     "   - Test the authentication"
  buh_line     "   - Initialize depot tracking"
  buh_line     "   - Reset the 6-month expiration timer"
  buh_e
  buh_section  "3. Verify Installation:"
  buh_line     "   Test with a simple operation:"
  buc_tabtarget "${RBZ_LIST_DEPOT}"
  buh_line     "   Should display current depots without authentication errors."
  buh_e
  buh_line     "Prevention: Run any Payor operation monthly to prevent expiration."

}

rbhp_quota_build() {
  zrbhp_sentinel

  buc_doc_brief "Display the Cloud Build capacity review procedure to verify machine type and quota settings"
  buc_doc_shown || return 0

  local z_ui z_cmd z_href

  buh_section  "Cloud Build Concurrent Build Capacity"
  buh_line     "Review your build capacity settings to ensure sufficient concurrent build execution."
  buh_line     "Recipe Bottle uses a private worker pool — quota is tracked per private pool host project."
  buh_e
  buh_line     "   Private pool machine types vs concurrency at 10-CPU quota:"
  buyy_cmd_yawp "(2 vCPU)   → 5 concurrent builds"; z_cmd="${z_buym_yelp}"
  buh_line     "     e2-standard-2  ${z_cmd}"
  buyy_cmd_yawp "(8 vCPU)   → 1 concurrent build"; z_cmd="${z_buym_yelp}"
  buh_line     "     e2-standard-8  ${z_cmd}"
  buyy_cmd_yawp "(32 vCPU)  → needs 32+ CPU quota"; z_cmd="${z_buym_yelp}"
  buh_line     "     e2-standard-32 ${z_cmd}"
  buh_e
  buh_section  "Key:"
  buyy_ui_yawp "precise words you see on the web page."; z_ui="${z_buym_yelp}"
  buh_line     "   Magenta text refers to ${z_ui}"
  buyy_cmd_yawp "something you might copy from here."; z_cmd="${z_buym_yelp}"
  buh_line     "   Cyan text is ${z_cmd}"
  buyy_href_yawp "https://example.com/" "EXAMPLE DOT COM"; z_href="${z_buym_yelp}"
  buh_line     "   Clickable links look like ${z_href} (often, ${ZRBHP_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Current Regime Configuration:"
  buyy_cmd_yawp "${RBRR_DEPOT_PROJECT_ID}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRR_DEPOT_PROJECT_ID:          ${z_cmd}"
  buyy_cmd_yawp "${RBRR_GCP_REGION}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRR_GCP_REGION:                ${z_cmd}"
  buyy_cmd_yawp "${RBRR_GCB_MACHINE_TYPE}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRR_GCB_MACHINE_TYPE:          ${z_cmd}"
  buyy_cmd_yawp "${RBRR_GCB_POOL_STEM}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRR_GCB_POOL_STEM:             ${z_cmd}"
  buyy_cmd_yawp "${RBRR_GCB_MIN_CONCURRENT_BUILDS}"; z_cmd="${z_buym_yelp}"
  buh_line     "   RBRR_GCB_MIN_CONCURRENT_BUILDS: ${z_cmd}"
  buh_e
  buh_line     "   The build preflight gate checks quota automatically before each build."
  buh_line     "   It computes: quota_vCPUs / machine_vCPUs >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  buh_e
  buh_section  "2. Check CPU Quota:"
  buh_line     "   Private pool quota is tracked under the depot project."
  buh_e
  buyy_href_yawp "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}" "Quotas & System Limits (opens to depot project)"; z_href="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href}"
  buyy_ui_yawp "${RBRR_DEPOT_PROJECT_ID}"; z_ui="${z_buym_yelp}"
  buh_line     "   1. Verify project ${z_ui} is selected in the project picker"
  buyy_ui_yawp "Enter property name or value"; z_ui="${z_buym_yelp}"
  buh_line     "   2. In the ${z_ui} filter bar, type:"
  buyy_cmd_yawp "concurrent_private"; z_cmd="${z_buym_yelp}"
  buh_line     "      ${z_cmd}"
  buyy_ui_yawp "cloudbuild.googleapis.com/concurrent_private_pool_build_cpus"; z_ui="${z_buym_yelp}"
  buh_line     "   3. Select ${z_ui} from the autocomplete"
  buh_line     "   4. Multiple rows appear. Look for the row with Type column showing"
  buyy_ui_yawp "Quota"; z_ui="${z_buym_yelp}"
  buh_line     "      ${z_ui} (not System limit) and your region in the Dimensions column"
  buh_line     "   5. Note the quota value and current usage percentage"
  buh_line     "      If usage is near 100% with one build, the machine type is too large for the quota"
  buh_e
  buh_section  "3. Request a Quota Increase (if needed):"
  buyy_ui_yawp "Quota"; z_ui="${z_buym_yelp}"
  buh_line     "   On the ${z_ui} row identified above:"
  buyy_ui_yawp "⋮"; z_ui="${z_buym_yelp}"
  buh_line     "   1. Click the three-dot menu ${z_ui} at the right end of the row"
  buyy_ui_yawp "Edit quota"; z_ui="${z_buym_yelp}"
  buh_line     "   2. Select ${z_ui}"
  buyy_ui_yawp "New value"; z_ui="${z_buym_yelp}"
  buh_line     "   3. In the side panel, enter the new value in the ${z_ui} field"
  buh_line     "      Recommended: 10 (allows 5 concurrent e2-standard-2 builds across both pools)"
  buyy_ui_yawp "Request description"; z_ui="${z_buym_yelp}"
  buh_line     "   4. A ${z_ui} field appears. Enter:"
  buyy_cmd_yawp "Need parallel builds on private worker pool for CI/CD pipeline testing."; z_cmd="${z_buym_yelp}"
  buh_line     "      ${z_cmd}"
  buyy_ui_yawp "Next"; z_ui="${z_buym_yelp}"
  buh_line     "   5. Click ${z_ui}"
  buh_line     "   6. Step 2/2 shows contact details (pre-filled from your Google account)"
  buyy_ui_yawp "Submit request"; z_ui="${z_buym_yelp}"
  buh_line     "   7. Click ${z_ui}"
  buh_line     "      Increases are typically approved within minutes."
  buh_e
  buh_section  "4. Confirm Quota Increase:"
  buh_line     "   After approval, quotas can take up to 15 minutes to propagate."
  buyy_href_yawp "https://console.cloud.google.com/iam-admin/quotas?project=${RBRR_DEPOT_PROJECT_ID}" "Quotas & System Limits (opens to depot project)"; z_href="${z_buym_yelp}"
  buh_line     "   Return to: ${z_href}"
  buyy_ui_yawp "concurrent_private"; z_ui="${z_buym_yelp}"
  buh_line     "   Filter for ${z_ui} again and verify the new value"
  buh_line     "   Verify: quota / vCPUs per machine type >= RBRR_GCB_MIN_CONCURRENT_BUILDS"
  buyy_cmd_yawp "${RBRR_GCB_MIN_CONCURRENT_BUILDS} concurrent builds"; z_cmd="${z_buym_yelp}"
  buh_line     "     Current target: ${z_cmd}"
  buh_e

}

# eof
