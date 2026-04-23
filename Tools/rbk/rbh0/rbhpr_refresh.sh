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
# Recipe Bottle Handbook Payor - Refresh ceremony function

set -euo pipefail

test -z "${ZRBHPR_SOURCED:-}" || return 0
ZRBHPR_SOURCED=1

rbhp_refresh() {
  zrbhp_sentinel

  buc_doc_brief "Display the manual Payor OAuth credential installation/refresh procedure"
  buc_doc_shown || return 0

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
  buyy_href_yawp "https://console.cloud.google.com/apis/credentials?project=${RBRP_PAYOR_PROJECT_ID}" "Credentials for Payor Project"; local -r z_href_credentials="${z_buym_yelp}"
  buh_line     "      - Go to: ${z_href_credentials}"
  buyy_ui_yawp "${RBGC_PAYOR_APP_NAME}"; local -r z_ui_app_name="${z_buym_yelp}"
  buh_line     "      - Find existing ${z_ui_app_name} OAuth client"
  buh_line     "      - Click the OAuth client name to open details"
  buh_line     "      - To rotate secret if compromised:"
  buyy_ui_yawp "+ Add secret"; local -r z_ui_add_secret="${z_buym_yelp}"
  buh_line     "        a. Click ${z_ui_add_secret}"
  buh_line     "        b. Click the download icon next to the NEW secret"
  buyy_ui_yawp "client_secret_[id].apps.googleusercontent.com.json"; local -r z_ui_download_filename="${z_buym_yelp}"
  buh_line     "           Browser downloads: ${z_ui_download_filename}"
  buyy_ui_yawp "Disable"; local -r z_ui_disable="${z_buym_yelp}"
  buh_line     "        c. Click ${z_ui_disable} on the secret with the older creation date"
  buh_line     "        d. Click the trash icon to delete that disabled secret"
  buh_e
  buh_section  "2. Install/Refresh OAuth Credentials:"
  buh_line     "   Run the payor install command with the downloaded JSON:"
  buh_tt       "      " "${RBZ_PAYOR_INSTALL}" "" " ~/Downloads/client_secret_*.json"
  buh_line     "   This will:"
  buh_line     "   - Guide you through OAuth authorization flow"
  buh_line     "   - Store secure credentials in RBRR_SECRETS_DIR/payor/rbro.env"
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

# eof
