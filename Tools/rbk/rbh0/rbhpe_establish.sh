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
# Recipe Bottle Handbook Payor - Establish ceremony function

set -euo pipefail

test -z "${ZRBHPE_SOURCED:-}" || return 0
ZRBHPE_SOURCED=1

rbhp_establish() {
  zrbhp_sentinel

  buc_doc_brief "Display the manual Payor Establishment procedure for OAuth authentication"
  buc_doc_shown || return 0

  buh_section  "Manual Payor OAuth Establishment Procedure"
  buh_line     "${RBGC_PAYOR_APP_NAME} now uses OAuth 2.0 for individual developer accounts."
  buh_line     "This resolves project creation limitations for personal Google accounts."
  buh_e
  buh_section  "Key:"
  buyy_ui_yawp "precise words you see on the web page."; local -r z_ui_key_words="${z_buym_yelp}"
  buh_line     "   Magenta text refers to ${z_ui_key_words}"
  buyy_cmd_yawp "something you might copy from here."; local -r z_cmd_key_copy="${z_buym_yelp}"
  buh_line     "   Cyan text is ${z_cmd_key_copy}"
  buyy_href_yawp "https://example.com/" "EXAMPLE DOT COM"; local -r z_href_key_example="${z_buym_yelp}"
  buh_line     "   Clickable links look like ${z_href_key_example} (often, ${ZRBHP_CLICK_MOD} + mouse click)"
  buh_e
  buh_section  "1. Confirm Payor Regime:"
  buyy_cmd_yawp "${ZRBHP_RBRP_FILE}"; local -r z_cmd_rbrp_file="${z_buym_yelp}"
  buh_line     "   File: ${z_cmd_rbrp_file}"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; local -r z_cmd_payor_project_id="${z_buym_yelp}"
  buh_line     "   RBRP_PAYOR_PROJECT_ID: ${z_cmd_payor_project_id}"
  buh_line     "   (You will discover RBRP_BILLING_ACCOUNT_ID later in step 5)"
  buh_e
  buh_line     "   First time setup? Set a timestamped project ID with:"
  buh_code     "   sed -i '' 's/^RBRP_PAYOR_PROJECT_ID=.*/RBRP_PAYOR_PROJECT_ID=${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-$(date "${RBGC_GLOBAL_TIMESTAMP_FORMAT}")/' ${ZRBHP_RBRP_FILE}"
  buh_e
  buh_section  "2. Check if Project Already Exists:"
  buh_line     "   Before creating a new project, verify the configured ID is not already in use:"
  buyy_href_yawp "https://console.cloud.google.com/cloud-resource-manager" "Google Cloud Project List"; local -r z_href_project_list="${z_buym_yelp}"
  buh_line     "   1. Check existing projects: ${z_href_project_list}"
  buyy_ui_yawp "${RBRP_PAYOR_PROJECT_ID}"; local -r z_ui_payor_project_id="${z_buym_yelp}"
  buh_line     "   2. Look for a project with ID ${z_ui_payor_project_id}"
  buh_line     "      - Hover over project IDs to verify the full ID matches your configured value"
  buyy_cmd_yawp "find the project"; local -r z_cmd_find_project="${z_buym_yelp}"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE_BASENAME}"; local -r z_ui_rbrp_file_basename="${z_buym_yelp}"
  buh_line     "   3. If you ${z_cmd_find_project} with matching ID, it already exists - edit ${z_ui_rbrp_file_basename}"
  buh_line     "      and re-run this procedure"
  buh_line     "   4. If you don't find it, proceed to step 3 to create it"
  buh_e
  buh_section  "3. Create Payor Project:"
  buyy_href_yawp "https://console.cloud.google.com/projectcreate" "Google Cloud Project Create"; local -r z_href_project_create="${z_buym_yelp}"
  buh_line     "   1. Open browser to: ${z_href_project_create}"
  buh_line     "   2. Ensure signed in with intended Google account (check top-right avatar)"
  buh_line     "   3. Configure new project:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; local -r z_cmd_app_name_create="${z_buym_yelp}"
  buh_line     "      - Project name: ${z_cmd_app_name_create}"
  buh_line     "      - Project ID: Google will auto-generate a value; click Edit to replace it with:"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; local -r z_cmd_payor_project_id_edit="${z_buym_yelp}"
  buh_line     "        ${z_cmd_payor_project_id_edit}"
  buyy_ui_yawp "No organization"; local -r z_ui_no_organization="${z_buym_yelp}"
  buh_line     "      - Location: ${z_ui_no_organization} (required for this guide; organization affiliation is advanced)"
  buyy_ui_yawp "CREATE"; local -r z_ui_create_project="${z_buym_yelp}"
  buh_line     "   4. Click ${z_ui_create_project}"
  buyy_ui_yawp "Creating project..."; local -r z_ui_creating_project="${z_buym_yelp}"
  buh_line     "   5. Wait for ${z_ui_creating_project} notification to complete"
  buh_e
  buh_section  "4. Verify Project Creation:"
  buh_line     "   Verify that your rbrp.env configuration matches the created project:"
  buyy_href_yawp "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}" "Google Cloud APIs Dashboard"; local -r z_href_apis_dashboard_verify="${z_buym_yelp}"
  buh_line     "   1. Test this link: ${z_href_apis_dashboard_verify}"
  buh_line     "   2. If the page loads and shows your project, your configuration is correct"
  buyy_ui_yawp "You need additional access"; local -r z_ui_need_additional_access="${z_buym_yelp}"
  buh_line     "   3. If you see ${z_ui_need_additional_access}, wait a few minutes and refresh the page"
  buyy_href_yawp "https://cloud.google.com/iam/docs/access-change-propagation" "Access Change Propagation"; local -r z_href_access_propagation="${z_buym_yelp}"
  buh_line     "      GCP IAM changes are eventually consistent: ${z_href_access_propagation}"
  buh_e
  buh_section  "5. Configure Billing Account:"
  buyy_href_yawp "https://console.cloud.google.com/billing" "Google Cloud Billing"; local -r z_href_billing="${z_buym_yelp}"
  buh_line     "   1. Go to: ${z_href_billing}"
  buh_line     "      If no billing accounts exist:"
  buyy_ui_yawp "CREATE ACCOUNT"; local -r z_ui_create_account="${z_buym_yelp}"
  buh_line     "          a. Click ${z_ui_create_account}"
  buh_line     "          b. Configure payment method and submit"
  buyy_ui_yawp "Account ID"; local -r z_ui_account_id_new="${z_buym_yelp}"
  buh_line     "          c. Copy new ${z_ui_account_id_new} from table"
  buh_line     "      else if single Open account exists:"
  buyy_ui_yawp "Account ID"; local -r z_ui_account_id_single="${z_buym_yelp}"
  buh_line     "          a. Copy the ${z_ui_account_id_single} value"
  buh_line     "      else if multiple Open accounts exist:"
  buh_line     "          a. Choose account for Recipe Bottle funding"
  buyy_ui_yawp "Account ID"; local -r z_ui_account_id_chosen="${z_buym_yelp}"
  buh_line     "          b. Copy chosen ${z_ui_account_id_chosen} value"
  buyy_href_yawp "https://console.cloud.google.com/billing/projects" "Google Cloud Billing Projects"; local -r z_href_billing_projects="${z_buym_yelp}"
  buh_line     "   2. Go to: ${z_href_billing_projects}"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE}"; local -r z_ui_rbrp_file_save="${z_buym_yelp}"
  buh_line     "   3. Save the billing account ID to your ${z_ui_rbrp_file_save}"
  buyy_cmd_yawp "RBRP_BILLING_ACCOUNT_ID="; local -r z_cmd_billing_account_id="${z_buym_yelp}"
  buyy_ui_yawp "Value from Account ID column"; local -r z_ui_account_id_value="${z_buym_yelp}"
  buh_line     "      Record as: ${z_cmd_billing_account_id} # ${z_ui_account_id_value}"
  buh_line     "   4. Find project row with ID matching your payor project (not name) and get the Account ID value"
  buyy_ui_yawp "${ZRBHP_RBRP_FILE}"; local -r z_ui_rbrp_file_update="${z_buym_yelp}"
  buh_line     "   5. Update ${z_ui_rbrp_file_update} and re-display this procedure."
  buh_e
  buh_section  "6. Link Billing to Payor Project:"
  buh_line     "   Link your billing account to the newly created project."
  buyy_href_yawp "https://console.cloud.google.com/billing/manage" "Google Cloud Billing Account Management"; local -r z_href_billing_manage="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href_billing_manage}"
  buh_line     "   1. The page loads to your default billing account."
  buyy_ui_yawp "Select a billing account"; local -r z_ui_select_billing_account="${z_buym_yelp}"
  buh_line     "   2. If you have multiple billing accounts, use the ${z_ui_select_billing_account} dropdown at top"
  buh_line     "      - Choose the account matching your RBRP_BILLING_ACCOUNT_ID"
  buyy_ui_yawp "Projects linked to this billing account"; local -r z_ui_projects_linked="${z_buym_yelp}"
  buh_line     "   3. Look for the section ${z_ui_projects_linked}"
  buh_line     "   4. Verify your payor project appears in the table:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; local -r z_cmd_app_name_verify="${z_buym_yelp}"
  buh_line     "      - Project name: ${z_cmd_app_name_verify}"
  buyy_cmd_yawp "${RBRP_PAYOR_PROJECT_ID}"; local -r z_cmd_payor_project_id_verify="${z_buym_yelp}"
  buh_line     "      - Project ID: ${z_cmd_payor_project_id_verify}"
  buh_line     "   5. If project is NOT listed and billing needs to be enabled:"
  buyy_href_yawp "https://console.cloud.google.com/billing/linkedaccount?project=${RBRP_PAYOR_PROJECT_ID}" "Project Billing"; local -r z_href_project_billing="${z_buym_yelp}"
  buh_line     "      - Go to: ${z_href_project_billing}"
  buyy_ui_yawp "Link a Billing Account"; local -r z_ui_link_billing_account="${z_buym_yelp}"
  buh_line     "      - Click ${z_ui_link_billing_account}"
  buh_line     "      - Select your billing account and confirm"
  buh_e
  buh_section  "7. Enable Required APIs:"
  buyy_href_yawp "https://console.cloud.google.com/apis/dashboard?project=${RBRP_PAYOR_PROJECT_ID}" "APIs & Services for your payor project"; local -r z_href_apis_dashboard_enable="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href_apis_dashboard_enable}"
  buyy_ui_yawp "+ ENABLE APIS AND SERVICES"; local -r z_ui_enable_apis="${z_buym_yelp}"
  buh_line     "   1. Click ${z_ui_enable_apis}"
  buh_line     "   2. Search for and enable these APIs:"
  buyy_cmd_yawp "Cloud Resource Manager API"; local -r z_cmd_resource_manager_api="${z_buym_yelp}"
  buh_line     "      - ${z_cmd_resource_manager_api}"
  buyy_cmd_yawp "Cloud Billing API"; local -r z_cmd_billing_api="${z_buym_yelp}"
  buh_line     "      - ${z_cmd_billing_api}"
  buyy_cmd_yawp "Service Usage API"; local -r z_cmd_service_usage_api="${z_buym_yelp}"
  buh_line     "      - ${z_cmd_service_usage_api} (often enabled by default, look for green check)"
  buyy_cmd_yawp "IAM Service Account Credentials API"; local -r z_cmd_iam_sa_credentials_api="${z_buym_yelp}"
  buh_line     "      - ${z_cmd_iam_sa_credentials_api}"
  buyy_cmd_yawp "Artifact Registry API"; local -r z_cmd_artifact_registry_api="${z_buym_yelp}"
  buh_line     "      - ${z_cmd_artifact_registry_api}"
  buh_line     "   These enable programmatic depot management operations."
  buh_e
  buh_section  "8. Configure OAuth Consent Screen:"
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials/consent?project=${RBRP_PAYOR_PROJECT_ID}" "OAuth consent screen"; local -r z_href_oauth_consent="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href_oauth_consent}"
  buyy_ui_yawp "Google Auth Platform not configured yet"; local -r z_ui_auth_not_configured="${z_buym_yelp}"
  buh_line     "   1. The console displays ${z_ui_auth_not_configured}"
  buyy_ui_yawp "Get started"; local -r z_ui_get_started="${z_buym_yelp}"
  buh_line     "   2. Click ${z_ui_get_started}"
  buh_line     "   3. Complete the Project Configuration wizard:"
  buh_line     "      Step 1 - App Information:"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; local -r z_cmd_app_name_consent="${z_buym_yelp}"
  buh_line     "        - App name: ${z_cmd_app_name_consent}"
  buh_line     "        - User support email: (your email)"
  buyy_ui_yawp "Next"; local -r z_ui_next_step1="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui_next_step1}"
  buh_line     "      Step 2 - Audience:"
  buyy_ui_yawp "External"; local -r z_ui_external="${z_buym_yelp}"
  buh_line     "        - Select ${z_ui_external}"
  buyy_ui_yawp "Next"; local -r z_ui_next_step2="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui_next_step2}"
  buh_line     "      Step 3 - Contact Information:"
  buh_line     "        - Email addresses: (your email), press Enter"
  buyy_ui_yawp "Next"; local -r z_ui_next_step3="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui_next_step3}"
  buh_line     "      Step 4 - Finish:"
  buyy_ui_yawp "I agree to the Google API Services: User Data Policy"; local -r z_ui_agree_policy="${z_buym_yelp}"
  buh_line     "        - Check ${z_ui_agree_policy}"
  buyy_ui_yawp "Continue"; local -r z_ui_continue="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui_continue}"
  buyy_ui_yawp "Create"; local -r z_ui_create_consent="${z_buym_yelp}"
  buh_line     "        - Click ${z_ui_create_consent}"
  buh_e
  buh_line     "   4. Add your email as a test user (avoids Google app verification process):"
  buyy_ui_yawp "Audience"; local -r z_ui_audience="${z_buym_yelp}"
  buh_line     "      1. Click ${z_ui_audience} in left sidebar"
  buyy_ui_yawp "Test users"; local -r z_ui_test_users="${z_buym_yelp}"
  buh_line     "      2. Scroll down to section ${z_ui_test_users}"
  buyy_ui_yawp "+ Add users"; local -r z_ui_add_users_button="${z_buym_yelp}"
  buh_line     "      3. Click ${z_ui_add_users_button}"
  buyy_ui_yawp "Add Users"; local -r z_ui_add_users_panel="${z_buym_yelp}"
  buh_line     "      4. Right-side panel titled ${z_ui_add_users_panel} slides in"
  buh_line     "      5. Enter your email address in the field"
  buyy_ui_yawp "Save"; local -r z_ui_save="${z_buym_yelp}"
  buh_line     "      6. Click ${z_ui_save}"
  buyy_ui_yawp "1 user"; local -r z_ui_one_user="${z_buym_yelp}"
  buh_line     "      7. Verify ${z_ui_one_user} appears in OAuth user cap"
  buh_e
  buh_section  "9. Create OAuth 2.0 Client ID:"
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}" "Credentials"; local -r z_href_credentials="${z_buym_yelp}"
  buh_line     "   Go to: ${z_href_credentials}"
  buyy_ui_yawp "+ Create credentials"; local -r z_ui_create_credentials="${z_buym_yelp}"
  buh_line     "   1. From top bar, click ${z_ui_create_credentials}"
  buyy_ui_yawp "OAuth client ID"; local -r z_ui_oauth_client_id="${z_buym_yelp}"
  buh_line     "   2. Select ${z_ui_oauth_client_id}"
  buyy_ui_yawp "Desktop app"; local -r z_ui_desktop_app="${z_buym_yelp}"
  buh_line     "   3. Application type: ${z_ui_desktop_app}"
  buyy_cmd_yawp "${RBGC_PAYOR_APP_NAME}"; local -r z_cmd_app_name_client="${z_buym_yelp}"
  buh_line     "   4. Name: ${z_cmd_app_name_client}"
  buyy_ui_yawp "CREATE"; local -r z_ui_create_client="${z_buym_yelp}"
  buh_line     "   5. Click ${z_ui_create_client}"
  buyy_ui_yawp "OAuth client created"; local -r z_ui_oauth_client_created="${z_buym_yelp}"
  buh_line     "   6. Popup titled ${z_ui_oauth_client_created} displays client ID and secret"
  buyy_ui_yawp "Download JSON"; local -r z_ui_download_json="${z_buym_yelp}"
  buh_line     "   7. Click ${z_ui_download_json}"
  buyy_ui_yawp "OK"; local -r z_ui_ok="${z_buym_yelp}"
  buyy_ui_yawp "client_secret_[id].apps.googleusercontent.com.json"; local -r z_ui_client_secret_filename="${z_buym_yelp}"
  buh_line     "   8. Click ${z_ui_ok} ; browser downloads ${z_ui_client_secret_filename}"
  buyy_warn_yawp "CRITICAL: Save securely - contains client secret"; local -r z_warn_save_securely="${z_buym_yelp}"
  buh_line     "      ${z_warn_save_securely}"
  buh_e
  buh_section  "10. Install OAuth Credentials:"
  buh_line     "   Run:"
  buh_code     "   rbgp_payor_install ~/Downloads/payor-oauth.json"
  buh_line     "   This will guide you through OAuth authorization and complete the setup."

}

# eof
