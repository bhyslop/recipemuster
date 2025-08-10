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

  # Define ANSI color codes
  ZRBMP_COLOR_RESET="\033[0m"
  ZRBMP_COLOR_SECTION="\033[1;37m"   # Bright white for sections
  ZRBMP_COLOR_COMMAND="\033[36m"     # Cyan for commands to type
  ZRBMP_COLOR_WEBSITE="\033[35m"     # Magenta for website text
  ZRBMP_COLOR_WARNING="\033[1;33m"   # Bright yellow for warnings
  ZRBMP_COLOR_CRITICAL="\033[1;31m"  # Bright red for critical warnings

  ZRBMP_KINDLED=1
}

zrbmp_sentinel() {
  test "${ZRBMP_KINDLED:-}" = "1" || bcu_die "Module rbmp not kindled - call zrbmp_kindle first"
}

zrbmp_show() {
  zrbmp_sentinel
  echo -e "${1}"
}

zrbmp_s1() { zrbmp_show "${ZRBMP_COLOR_SECTION}# ${1}${ZRBMP_COLOR_RESET}"; }
zrbmp_s2() { zrbmp_show "${ZRBMP_COLOR_SECTION}## ${1}${ZRBMP_COLOR_RESET}"; }
zrbmp_s3() { zrbmp_show "${ZRBMP_COLOR_SECTION}### ${1}${ZRBMP_COLOR_RESET}"; }

zrbmp_e()  { zrbmp_show ""; } # Empty line

zrbmp_n() {
  zrbmp_sentinel
  echo "${1}"
}

zrbmp_nc() {
  zrbmp_sentinel
  # Normal text + command
  echo -e "${1}${ZRBMP_COLOR_COMMAND}${2}${ZRBMP_COLOR_RESET}"
}

zrbmp_nw() {
  zrbmp_sentinel
  # Normal text + website text
  echo -e "${1}${ZRBMP_COLOR_WEBSITE}${2}${ZRBMP_COLOR_RESET}"
}

zrbmp_nwn() {
  zrbmp_sentinel
  # Normal + website + normal
  echo -e "${1}${ZRBMP_COLOR_WEBSITE}${2}${ZRBMP_COLOR_RESET}${3}"
}

zrbmp_nwnw() {
  zrbmp_sentinel
  # Normal + website + normal + website
  echo -e "${1}${ZRBMP_COLOR_WEBSITE}${2}${ZRBMP_COLOR_RESET}${3}${ZRBMP_COLOR_WEBSITE}${4}${ZRBMP_COLOR_RESET}"
}

zrbmp_ncn() {
  zrbmp_sentinel
  # Normal + command + normal
  echo -e "${1}${ZRBMP_COLOR_COMMAND}${2}${ZRBMP_COLOR_RESET}${3}"
}

zrbmp_cmd() {
  zrbmp_sentinel
  echo -e "${ZRBMP_COLOR_COMMAND}${1}${ZRBMP_COLOR_RESET}"
}

zrbmp_warning() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_WARNING}⚠️  WARNING: ${1}${ZRBMP_COLOR_RESET}\n"
}

zrbmp_critic() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_CRITICAL}🔴 CRITICAL SECURITY WARNING: ${1}${ZRBMP_COLOR_RESET}\n"
}

######################################################################
# External Functions (rbmp_*)

rbmp_show_setup() {
  zrbmp_sentinel

  bcu_doc_brief "Display the manual GCP provisioner setup procedure"
  bcu_doc_shown || return 0

  zrbmp_s1   "Google Cloud Platform Setup"
  
  zrbmp_s2 "Overview"
  zrbmp_n      "Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges."
  zrbmp_n      "The provisioner will automate the creation of operational service accounts and infrastructure configuration."
  
  zrbmp_s2 "Prerequisites"
  zrbmp_n      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbmp_n      "- Email address not already associated with GCP"
  zrbmp_e
  zrbmp_n      "---"
  
  zrbmp_s1   "Manual Provisioner Setup Procedure"
  
  zrbmp_n      "Recipe Bottle setup requires a manual bootstrap procedure to enable enough control"
  zrbmp_e
  zrbmp_nc     "Open a web browser to " "https://cloud.google.com/free"
  zrbmp_e
  
  zrbmp_n      "1. **Establish Account**"
  zrbmp_nw     "   1. Click ->" "Get started for free"
  zrbmp_n      "   1. Sign in with Google account or create new"
  zrbmp_n      "   1. Provide:"
  zrbmp_n      "      - Country"
  zrbmp_nw     "      - Organization type: " "Individual"
  zrbmp_n      "      - Credit card (verification only)"
  zrbmp_nw     "   1. Accept terms → " "Start my free trial"
  zrbmp_n      "   1. Expect Google Cloud Console to open."
  
  zrbmp_n      "1. **Create New Project**"
  zrbmp_nw     "   1. Top bar project dropdown → " "New Project"
  zrbmp_n      "   1. Configure:"
  zrbmp_nc     "      - Project name: " "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}"
  zrbmp_nw     "      - Leave organization as -> " "No organization"
  zrbmp_nwn    "   1. Create → Wait for notification -> " "Creating project..." " to complete"
  zrbmp_n      "   1. Select project from dropdown when ready"
  
  zrbmp_n      "1. **Create Provisioner Service Account**"
  zrbmp_n      "   1. Navigate to IAM & Admin section"
  zrbmp_nw     "   1. Left sidebar → " "IAM & Admin → Service Accounts"
  zrbmp_nw     "   1. If prompted about APIs, click -> " "Enable API"
  zrbmp_nw     "   1. Wait for " "Identity and Access Management (IAM) API to enable"
  
  zrbmp_n      "1. **Create the Provisioner**"
  zrbmp_nw     "   1. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbmp_n      "   1. Service account details:"
  zrbmp_nc     "      - Service account name: " "rbra-provisioner"
  zrbmp_nwn    "      - Service account ID: (auto-fills as " "rbra-provisioner" ")"
  zrbmp_nc     "      - Description: " "Temporary provisioner for infrastructure setup - DELETE AFTER USE"
  zrbmp_nw     "   1. Click -> " "CREATE AND CONTINUE"
  
  zrbmp_n      "1. Assign Project Owner Role:"
  
  zrbmp_critic "This grants complete project control. Delete immediately after setup."
  
  zrbmp_n      "Grant access section:"
  zrbmp_nw     "1. Click dropdown " "Select a role"
  zrbmp_nc     "2. In filter box, type: " "owner"
  zrbmp_nw     "3. Select: " "Basic → Owner"
  zrbmp_nw     "4. Click -> " "CONTINUE"
  zrbmp_nw     "5. Grant users access section: Skip by clicking -> " "DONE"
  zrbmp_e
  zrbmp_nw     "Service account list now shows " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  
  zrbmp_s3 "4. Generate Service Account Key"
  
  zrbmp_n      "From service accounts list:"
  zrbmp_nw     "1. Click on " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  zrbmp_nw     "2. Top tabs → " "KEYS"
  zrbmp_nwnw   "3. Click " "ADD KEY"  → "Create new key"
  zrbmp_nw     "4. Key type: " "JSON (should be selected)"
  zrbmp_nw     "5. Click " "CREATE"
  zrbmp_e
  zrbmp_nw     "Browser downloads: " "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-[random].json"
  zrbmp_e
  zrbmp_nwn    "6. Click " "CLOSE" " on download confirmation"
  
  zrbmp_s3 "5. Configure Local Environment"
  
  zrbmp_n      "Open terminal ⟨LOCAL-SETUP⟩:"
  zrbmp_e
  zrbmp_cmd    "# Create secrets directory structure"
  zrbmp_cmd    "mkdir -p ../station-files/secrets"
  zrbmp_cmd    "cd ../station-files/secrets"
  zrbmp_e
  zrbmp_cmd    "# Move downloaded key (adjust path to your Downloads folder)"
  zrbmp_cmd    "mv ~/Downloads/${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-*.json rbra-provisioner-key.json"
  zrbmp_e
  zrbmp_cmd    "# Verify key structure"
  zrbmp_cmd    "jq -r '.type' rbra-provisioner-key.json"
  zrbmp_cmd    "# Should output: service_account"
  zrbmp_e
  zrbmp_cmd    "# Create RBRA environment file"
  zrbmp_cmd    "cat > rbra-provisioner.env << 'EOF'"
  zrbmp_cmd    "RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-provisioner-key.json"
  zrbmp_cmd    "RBRA_TOKEN_LIFETIME_SEC=1800"
  zrbmp_cmd    "EOF"
  zrbmp_e
  zrbmp_cmd    "# Set restrictive permissions"
  zrbmp_cmd    "chmod 600 rbra-provisioner-key.json"
  zrbmp_cmd    "chmod 600 rbra-provisioner.env"
  
  zrbmp_e
  
  zrbmp_warning "Remember to delete the provisioner service account after infrastructure setup is complete!"
  
  zrbmp_n      "The provisioner environment file is now configured at:"
  zrbmp_nc     "" "${RBRR_PROVISIONER_RBRA_FILE:-../station-files/secrets/rbra-provisioner.env}"
  zrbmp_e
  
  bcu_success "Manual setup procedure displayed"
}

rbmp_show_teardown() {
  zrbmp_sentinel

  bcu_doc_brief "Display the procedure for removing the provisioner after setup"
  bcu_doc_shown || return 0

  zrbmp_s1   "Provisioner Teardown Procedure"
  
  zrbmp_critic "Execute this immediately after infrastructure setup is complete!"
  
  zrbmp_n      "1. **Delete Service Account Key from GCP Console**"
  zrbmp_nw     "   1. Navigate to " "IAM & Admin → Service Accounts"
  zrbmp_nw     "   2. Click on " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  zrbmp_nw     "   3. Go to " "KEYS tab"
  zrbmp_n      "   4. Find the key created earlier"
  zrbmp_nw     "   5. Click the three dots menu → " "Delete"
  zrbmp_n      "   6. Confirm deletion"
  zrbmp_n      ""
  
  zrbmp_n      "2. **Delete Service Account**"
  zrbmp_n      "   1. Return to Service Accounts list"
  zrbmp_nw     "   2. Check the box next to " "rbra-provisioner"
  zrbmp_nw     "   3. Click " "DELETE at top"
  zrbmp_n      "   4. Type the confirmation text"
  zrbmp_nw     "   5. Click " "DELETE"
  zrbmp_n      ""
  
  zrbmp_n      "3. **Remove Local Files**"
  zrbmp_cmd    "# Remove provisioner credentials"
  zrbmp_cmd    "cd ../station-files/secrets"
  zrbmp_cmd    "shred -vuz rbra-provisioner-key.json"
  zrbmp_cmd    "shred -vuz rbra-provisioner.env"
  zrbmp_n      ""
  
  zrbmp_n      "4. **Verify Removal**"
  zrbmp_n      "   - Check GCP Console shows no rbra-provisioner service account"
  zrbmp_n      "   - Verify local files are removed:"
  zrbmp_cmd    "ls -la ../station-files/secrets/ | grep provisioner"
  zrbmp_n      "   (should return nothing)"
  zrbmp_n      ""
  
  bcu_success "Teardown procedure displayed"
}

