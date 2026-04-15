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
# Recipe Bottle Handbook Payor - Quota build ceremony function

set -euo pipefail

test -z "${ZRBHPQ_SOURCED:-}" || return 0
ZRBHPQ_SOURCED=1

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
