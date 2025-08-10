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
# Recipe Bottle Manual Procedures - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBMP_SOURCED:-}" || bcu_die "Module rbmp multiply sourced - check sourcing hierarchy"
ZRBMP_SOURCED=1

######################################################################
# Internal Functions (zrbmp_*)

zrbmp_kindle() {
  test -z "${ZRBMP_KINDLED:-}" || bcu_die "Module rbmp already kindled"

  # Check terminal color support and adjust accordingly
  if [ -t 1 ] && [ "${TERM}" != "dumb" ] && [ "${NO_COLOR:-}" = "" ]; then
    # Terminal supports colors
    ZRBMP_R="\033[0m"         # Reset
    ZRBMP_S="\033[1;37m"      # Section (bright white)
    ZRBMP_C="\033[36m"        # Command (cyan)
    ZRBMP_W="\033[35m"        # Website (magenta)
    ZRBMP_WN="\033[1;33m"     # Warning (bright yellow)
    ZRBMP_CR="\033[1;31m"     # Critical (bright red)
  else
    # No color support or colors disabled
    ZRBMP_R=""
    ZRBMP_S=""
    ZRBMP_C=""
    ZRBMP_W=""
    ZRBMP_WN=""
    ZRBMP_CR=""
  fi

  ZRBMP_PROVISIONER_ROLE="rbra-provisioner"

  ZRBMP_KINDLED=1
}

zrbmp_sentinel() { test "${ZRBMP_KINDLED:-}" = "1" || bcu_die "Module rbmp not kindled - call zrbmp_kindle first"; }

zrbmp_show() { zrbmp_sentinel; echo -e "${1:-}"; }

zrbmp_s1() { zrbmp_show "${ZRBMP_S}${1}${ZRBMP_R}"; }
zrbmp_s2() { zrbmp_show "${ZRBMP_S}${1}${ZRBMP_R}"; }
zrbmp_s3() { zrbmp_show "${ZRBMP_S}${1}${ZRBMP_R}"; }

zrbmp_e()    { zrbmp_show; }
zrbmp_n()    { zrbmp_show "${1}";                                                     }
zrbmp_nc()   { zrbmp_show "${1}${ZRBMP_C}${2}${ZRBMP_R}";                             }
zrbmp_ncn()  { zrbmp_show "${1}${ZRBMP_C}${2}${ZRBMP_R}${3}";                         }
zrbmp_nw()   { zrbmp_show "${1}${ZRBMP_W}${2}${ZRBMP_R}";                             }
zrbmp_nwn()  { zrbmp_show "${1}${ZRBMP_W}${2}${ZRBMP_R}${3}";                         }
zrbmp_nwnw() { zrbmp_show "${1}${ZRBMP_W}${2}${ZRBMP_R}${3}${ZRBMP_W}${4}${ZRBMP_R}"; }

zrbmp_ne()   { zrbmp_show "${1}${ZRBMP_CR}${2}${ZRBMP_R}"; }

zrbmp_cmd()     { zrbmp_show "${ZRBMP_C}${1}${ZRBMP_R}";                                 }
zrbmp_warning() { zrbmp_show "\n${ZRBMP_WN}⚠️  WARNING: ${1}${ZRBMP_R}\n"; }
zrbmp_critic()  { zrbmp_show "\n${ZRBMP_CR}🔴 CRITICAL SECURITY WARNING: ${1}${ZRBMP_R}\n"; }

######################################################################
# External Functions (rbmp_*)

rbmp_show_setup() {
  zrbmp_sentinel
  
  local z_rbrr_file="../station-files/rbrr_RecipeBottleRegimeRepo.sh"

  bcu_doc_brief "Display the manual GCP provisioner setup procedure"
  bcu_doc_shown || return 0

  zrbmp_s1     "# Google Cloud Platform Setup"
  zrbmp_s2     "## Overview"
  zrbmp_n      "Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges."
  zrbmp_n      "The provisioner will automate the creation of operational service accounts and infrastructure configuration."
  zrbmp_s2     "## Prerequisites"
  zrbmp_n      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbmp_n      "- Email address not already associated with GCP"
  zrbmp_e
  zrbmp_n      "---"
  zrbmp_s1     "Manual Provisioner Setup Procedure"
  zrbmp_n      "Recipe Bottle setup requires a manual bootstrap procedure to enable enough control"
  zrbmp_e
  zrbmp_nc     "Open a web browser to " "https://cloud.google.com/free"
  zrbmp_e
  zrbmp_s2     "1. Establish Account:"
  zrbmp_nw     "   1. Click -> " "Get started for free"
  zrbmp_n      "   2. Sign in with Google account or create new"
  zrbmp_n      "   3. Provide:"
  zrbmp_n      "      - Country"
  zrbmp_nw     "      - Organization type: " "Individual"
  zrbmp_n      "      - Credit card (verification only)"
  zrbmp_nw     "   4. Accept terms → " "Start my free trial"
  zrbmp_n      "   5. Expect Google Cloud Console to open."
  zrbmp_nwn    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbmp_e
  zrbmp_s2     "2. Configure Project ID and Region:"
  zrbmp_n      "   Before creating the project, you must choose a unique project ID."
  zrbmp_nc     "   1. Edit your RBRR configuration file: " "${z_rbrr_file}"
  zrbmp_n      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbmp_n      "      - Must be globally unique across all GCP"
  zrbmp_n      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbmp_n      "      - Cannot start/end with hyphen"
  zrbmp_nc     "      - Suggested pattern: " "yourname-recipemuster-proj"
  zrbmp_n      "   3. Choose RBRR_GCP_REGION based on your location:"
  zrbmp_nc     "      - US Central: " "us-central1 (default)"
  zrbmp_nc     "      - US East: " "us-east1"
  zrbmp_nc     "      - Europe: " "europe-west1"
  zrbmp_nc     "      - Asia: " "asia-northeast1"
  zrbmp_n      "   4. Save the file before proceeding"
  zrbmp_e
  zrbmp_s2     "3. Create New Project:"
  zrbmp_n      "   Confirm you're in the Google Cloud Console (not the marketing site)"
  zrbmp_nw     "   1. Top bar project dropdown → " "New Project"
  zrbmp_n      "   2. Configure:"
  zrbmp_nc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbmp_nw     "      - Leave organization as -> " "No organization"
  zrbmp_nw     "   3. Click " "CREATE"
  zrbmp_e
  zrbmp_ne     "   ERROR: If \"The project ID is already taken\":" " STOP HERE"
  zrbmp_n      "         1. Cancel project creation"
  zrbmp_nc     "         2. Edit " "${z_rbrr_file}" " with a different project ID"
  zrbmp_n      "         3. Restart from Step 3"
  zrbmp_e
  zrbmp_nwn    "   4. Wait for notification -> " "Creating project..." " to complete"
  zrbmp_n      "   5. Select project from dropdown when ready"
  zrbmp_e
  zrbmp_s2     "4. Navigate to Service Accounts:"
  zrbmp_n      "   1. Ensure your new project is selected in the top dropdown"
  zrbmp_nw     "   2. Left sidebar → " "IAM & Admin → Service Accounts"
  zrbmp_nw     "   3. If prompted about APIs, click -> " "Enable API"
  zrbmp_nw     "   4. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbmp_e
  zrbmp_s2     "5. Create the Provisioner:"
  zrbmp_nw     "   1. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbmp_n      "   2. Service account details:"
  zrbmp_nc     "      - Service account name: " "${ZRBMP_PROVISIONER_ROLE}"
  zrbmp_nwn    "      - Service account ID: (auto-fills as " "${ZRBMP_PROVISIONER_ROLE}" ")"
  zrbmp_nc     "      - Description: " "Temporary provisioner for infrastructure setup - DELETE AFTER USE"
  zrbmp_nw     "   3. Click -> " "CREATE AND CONTINUE"
  zrbmp_e
  zrbmp_s2     "6. Assign Project Owner Role:"
  zrbmp_critic "This grants complete project control. Delete immediately after setup."
  zrbmp_n      "Grant access section:"
  zrbmp_nw     "   1. Click dropdown " "Select a role"
  zrbmp_nc     "   2. In filter box, type: " "owner"
  zrbmp_nw     "   3. Select: " "Basic → Owner"
  zrbmp_nw     "   4. Click -> " "CONTINUE"
  zrbmp_nw     "   5. Grant users access section: Skip by clicking -> " "DONE"
  zrbmp_e
  zrbmp_nw     "Service account list now shows " "${ZRBMP_PROVISIONER_ROLE}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"
  zrbmp_e
  zrbmp_s2     "7. Generate Service Account Key:"
  zrbmp_n      "From service accounts list:"
  zrbmp_nw     "   1. Click on " "${ZRBMP_PROVISIONER_ROLE}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"
  zrbmp_nw     "   2. Top tabs → " "KEYS"
  zrbmp_nwnw   "   3. Click " "ADD KEY"  " → " "Create new key"
  zrbmp_nw     "   4. Key type: " "JSON (should be selected)"
  zrbmp_nw     "   5. Click " "CREATE"
  zrbmp_e
  zrbmp_nw     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbmp_e
  zrbmp_nwn    "   6. Click " "CLOSE" " on download confirmation"
  zrbmp_e
  zrbmp_s2     "8. Configure Local Environment:"
  zrbmp_n      "Open terminal ⟨LOCAL-SETUP⟩:"
  zrbmp_e
  zrbmp_cmd    "# Create secrets directory structure"
  zrbmp_cmd    "mkdir -p ../station-files/secrets"
  zrbmp_cmd    "cd ../station-files/secrets"
  zrbmp_e
  zrbmp_cmd    "# Move downloaded key (adjust path to your Downloads folder)"
  zrbmp_cmd    "mv ~/Downloads/${RBRR_GCP_PROJECT_ID}-*.json ${ZRBMP_PROVISIONER_ROLE}-key.json"
  zrbmp_e
  zrbmp_cmd    "# Verify key structure"
  zrbmp_cmd    "jq -r '.type' ${ZRBMP_PROVISIONER_ROLE}-key.json"
  zrbmp_cmd    "# Should output: service_account"
  zrbmp_e
  zrbmp_cmd    "# Create RBRA environment file"
  zrbmp_cmd    "cat > ${ZRBMP_PROVISIONER_ROLE}.env << 'EOF'"
  zrbmp_cmd    "RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/${ZRBMP_PROVISIONER_ROLE}-key.json"
  zrbmp_cmd    "RBRA_TOKEN_LIFETIME_SEC=1800"
  zrbmp_cmd    "EOF"
  zrbmp_e
  zrbmp_cmd    "# Set restrictive permissions"
  zrbmp_cmd    "chmod 600 ${ZRBMP_PROVISIONER_ROLE}-key.json"
  zrbmp_cmd    "chmod 600 ${ZRBMP_PROVISIONER_ROLE}.env"
  zrbmp_e
  zrbmp_warning "Remember to delete the provisioner service account after infrastructure setup is complete!"
  zrbmp_n      "The provisioner environment file is now configured at:"
  zrbmp_nc     "" "${RBRR_PROVISIONER_RBRA_FILE:-../station-files/secrets/${ZRBMP_PROVISIONER_ROLE}.env}"
  zrbmp_e

  bcu_success "Manual setup procedure displayed"
}

rbmp_show_teardown() {
  zrbmp_sentinel

  bcu_doc_brief "Display the procedure for removing the provisioner after setup"
  bcu_doc_shown || return 0

  zrbmp_s1     "Provisioner Teardown Procedure"

  zrbmp_critic "Execute this immediately after infrastructure setup is complete!"

  zrbmp_s2     "1. Delete Service Account Key from GCP Console:"
  zrbmp_nw     "   1. Navigate to " "IAM & Admin → Service Accounts"
  zrbmp_nw     "   2. Click on " "${ZRBMP_PROVISIONER_ROLE}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"
  zrbmp_nw     "   3. Go to " "KEYS tab"
  zrbmp_n      "   4. Find the key created earlier"
  zrbmp_nw     "   5. Click the three dots menu → " "Delete"
  zrbmp_n      "   6. Confirm deletion"
  zrbmp_e
  zrbmp_s2     "2. Delete Service Account:"
  zrbmp_n      "   1. Return to Service Accounts list"
  zrbmp_nw     "   2. Check the box next to " "${ZRBMP_PROVISIONER_ROLE}"
  zrbmp_nw     "   3. Click " "DELETE at top"
  zrbmp_n      "   4. Type the confirmation text"
  zrbmp_nw     "   5. Click " "DELETE"
  zrbmp_e
  zrbmp_s2     "3. Remove Local Files:"
  zrbmp_cmd    "# Remove provisioner credentials"
  zrbmp_cmd    "cd ../station-files/secrets"
  zrbmp_cmd    "shred -vuz ${ZRBMP_PROVISIONER_ROLE}-key.json"
  zrbmp_cmd    "shred -vuz ${ZRBMP_PROVISIONER_ROLE}.env"
  zrbmp_e
  zrbmp_s2     "4. Verify Removal:"
  zrbmp_n      "   - Check GCP Console shows no ${ZRBMP_PROVISIONER_ROLE} service account"
  zrbmp_n      "   - Verify local files are removed:"
  zrbmp_cmd    "ls -la ../station-files/secrets/ | grep provisioner"
  zrbmp_n      "   (should return nothing)"
  zrbmp_e

  bcu_success "Teardown procedure displayed"
}

