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
# Recipe Bottle Handbook Payor - GCP organization founding guide function

set -euo pipefail

test -z "${ZRBHPO_SOURCED:-}" || return 0
ZRBHPO_SOURCED=1

rbhp_organization() {
  zrbhp_sentinel

  buc_doc_brief "Display the GCP organization founding procedure yielding RBRW_ORG_ID"
  buc_doc_shown || return 0

  # Several UI words (Verify, Organization) are yawped once and reused across
  # sections below; the readonly captures survive later buyy_*_yawp calls
  # overwriting the shared z_buym_yelp scratch.
  buh_section  "GCP Organization Founding Procedure"
  buh_line     "Founds the GCP organization that owns the manor's one workforce identity pool."
  buh_line     "The organization is a Google Cloud resource node created by a Cloud Identity"
  buh_line     "signup over a DNS domain you control; verifying the domain provisions it."
  buh_line     "Everything here is Google-side, human-only console work; the one value it yields"
  buh_line     "is the organization's numeric ID, which becomes RBRW_ORG_ID."
  buh_e
  buh_line     "SKIP THIS WHOLE GUIDE if you already administer a GCP organization — read its"
  buh_line     "numeric ID from step 4 and record it (step 5)."
  buh_e
  buh_section  "Key:"
  buyy_ui_yawp "precise words you see on the web page."; local -r z_ui_key_words="${z_buym_yelp}"
  buh_line     "   Magenta text refers to ${z_ui_key_words}"
  buyy_cmd_yawp "something you might copy from here."; local -r z_cmd_key_copy="${z_buym_yelp}"
  buh_line     "   Cyan text is ${z_cmd_key_copy}"
  buyy_href_yawp "https://example.com/" "EXAMPLE DOT COM"; local -r z_href_key_example="${z_buym_yelp}"
  buh_line     "   Clickable links look like ${z_href_key_example} (often, ${ZRBHP_CLICK_MOD} + mouse click)"
  buyy_cmd_yawp "«guillemets»"; local -r z_cmd_key_guillemet="${z_buym_yelp}"
  buh_line     "   Values in ${z_cmd_key_guillemet} are yours to substitute — never copy them literally"
  buh_e
  buh_section  "1. Confirm You Have a Domain:"
  buh_line     "   Cloud Identity founds the organization over a DNS domain you own and can add"
  buh_line     "   records to (e.g. through your registrar). You do not need a website — only"
  buh_line     "   authority to publish a DNS record on the domain. If you own no domain, register"
  buh_line     "   one at any registrar first; a bare personal Google account cannot found an"
  buh_line     "   organization without one."
  buh_e
  buh_section  "2. Sign Up for Cloud Identity (Free):"
  buyy_href_yawp "https://workspace.google.com/gcpidentity/signup" "Cloud Identity Free Signup"; local -r z_href_ci_signup="${z_buym_yelp}"
  buh_line     "   1. Open: ${z_href_ci_signup}"
  buh_line     "   2. Enter your business/contact details when asked"
  buyy_cmd_yawp "«your-domain»"; local -r z_cmd_domain="${z_buym_yelp}"
  buh_line     "   3. Provide the domain you control as ${z_cmd_domain} (e.g. example.com)"
  buh_line     "   4. Create the initial administrator sign-in for the new Cloud Identity account"
  buh_line     "   This is the free Cloud Identity edition — no Workspace subscription and no"
  buh_line     "   charge; it exists solely to provision the organization resource node."
  buh_e
  buh_section  "3. Verify Domain Ownership (DNS):"
  buyy_href_yawp "https://admin.google.com" "Google Admin Console"; local -r z_href_admin="${z_buym_yelp}"
  buh_line     "   1. In the ${z_href_admin} setup flow, choose to verify your domain"
  buyy_ui_yawp "TXT"; local -r z_ui_txt="${z_buym_yelp}"
  buh_line     "   2. Google shows a ${z_ui_txt} record value to publish"
  buh_line     "   3. At your DNS provider, add that ${z_ui_txt} record to ${z_cmd_domain}"
  buyy_ui_yawp "Verify"; local -r z_ui_verify="${z_buym_yelp}"
  buh_line     "   4. Return to the console and click ${z_ui_verify}"
  buyy_warn_yawp "DNS propagation can take minutes to hours — this step bakes"; local -r z_warn_bake="${z_buym_yelp}"
  buh_line     "      ${z_warn_bake}. You may leave it verifying and do other console work"
  buh_line     "      (the payor wrapper points you at the next guide meanwhile); return and"
  buh_line     "      re-click ${z_ui_verify} once the record has propagated."
  buh_e
  buh_line     "   Verifying the domain provisions the GCP organization automatically — the"
  buh_line     "   organization node appears once verification succeeds."
  buh_e
  buh_section  "4. Read the Organization Numeric ID:"
  buyy_href_yawp "https://console.cloud.google.com/cloud-resource-manager" "Cloud Resource Manager"; local -r z_href_crm="${z_buym_yelp}"
  buh_line     "   1. Open: ${z_href_crm} signed in as the Cloud Identity administrator"
  buyy_ui_yawp "Organization"; local -r z_ui_organization="${z_buym_yelp}"
  buh_line     "   2. At the top, the ${z_ui_organization} picker shows your domain as the org node"
  buyy_ui_yawp "ID"; local -r z_ui_id="${z_buym_yelp}"
  buh_line     "   3. Read its numeric ${z_ui_id} (all digits) beside the organization name"
  buh_line     "   The equivalent one-liner, if you prefer the CLI:"
  buh_code     "   gcloud organizations list"
  buh_e
  buh_section  "5. Record the Organization ID:"
  buyy_cmd_yawp "${RBCC_rbrw_file}"; local -r z_cmd_rbrw_file="${z_buym_yelp}"
  buh_line     "   File: ${z_cmd_rbrw_file}"
  buh_line     "   (the workforce regime — the manor's one workforce identity pool; it ships"
  buh_line     "   committed, every value a public identifier)"
  buh_line     "   Set the organization field from the numeric ID above:"
  buyy_cmd_yawp "RBRW_ORG_ID=«organization-numeric-id»"; local -r z_cmd_field_org="${z_buym_yelp}"
  buh_line     "   ${z_cmd_field_org}"
  buh_line     "   The pool ID and session duration in the same file are set later, at Instaurate,"
  buh_line     "   where the manor-setup finisher founds the pool from all three."
  buh_e
  buh_section  "6. Validate:"
  buh_line     "   Confirm the field is well-formed (numeric organization ID):"
  buh_tt       "      " "${RBZ_VALIDATE_WORKFORCE}" "" ""
  buh_line     "   The pool-id and session-duration checks also run; they pass once those are"
  buh_line     "   set at Instaurate. Every value is a public identifier, so the file ships"
  buh_line     "   committed — commit it with your usual workflow."
  buh_e

}

# eof
