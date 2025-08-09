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

  # Validate environment
  test -n "${BDU_TEMP_DIR:-}" || bcu_die "BDU_TEMP_DIR is unset or empty"
  test -n "${BDU_NOW_STAMP:-}" || bcu_die "BDU_NOW_STAMP is unset or empty"

  # Source RBL module to get RBRR file location
  zrbl_kindle
  bvu_file_exists "${RBL_RBRR_FILE}"
  source "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR regime file"

  # Define ANSI color codes
  ZRBMP_COLOR_RESET="\033[0m"
  ZRBMP_COLOR_SECTION="\033[1;37m"  # Bright white for sections
  ZRBMP_COLOR_COMMAND="\033[36m"     # Cyan for commands to type
  ZRBMP_COLOR_WEBSITE="\033[35m"     # Magenta for website text
  ZRBMP_COLOR_WARNING="\033[1;33m"   # Bright yellow for warnings
  ZRBMP_COLOR_CRITICAL="\033[1;31m"  # Bright red for critical warnings

  # Define file paths
  ZRBMP_TEMP_FILE="${BDU_TEMP_DIR}/rbmp_temp.txt"

  ZRBMP_KINDLED=1
}

zrbmp_sentinel() {
  test "${ZRBMP_KINDLED:-}" = "1" || bcu_die "Module rbmp not kindled - call zrbmp_kindle first"
}

# Display helper functions - deliberate BCG exception for readability
# These are display-only functions where inline color matters

zrbmp_section() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_SECTION}# ${1}${ZRBMP_COLOR_RESET}\n"
}

zrbmp_subsection() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_SECTION}## ${1}${ZRBMP_COLOR_RESET}\n"
}

zrbmp_subsubsection() {
  zrbmp_sentinel
  echo -e "${ZRBMP_COLOR_SECTION}### ${1}${ZRBMP_COLOR_RESET}\n"
}

zrbmp_normal() {
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

zrbmp_ncn() {
  zrbmp_sentinel
  # Normal + command + normal
  echo -e "${1}${ZRBMP_COLOR_COMMAND}${2}${ZRBMP_COLOR_RESET}${3}"
}

zrbmp_command() {
  zrbmp_sentinel
  echo -e "${ZRBMP_COLOR_COMMAND}${1}${ZRBMP_COLOR_RESET}"
}

zrbmp_warning() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_WARNING}⚠️  WARNING: ${1}${ZRBMP_COLOR_RESET}\n"
}

zrbmp_critical() {
  zrbmp_sentinel
  echo -e "\n${ZRBMP_COLOR_CRITICAL}🔴 CRITICAL SECURITY WARNING: ${1}${ZRBMP_COLOR_RESET}\n"
}

######################################################################
# External Functions (rbmp_*)

rbmp_show_setup() {
  zrbmp_sentinel

  bcu_doc_brief "Display the manual GCP provisioner setup procedure"
  bcu_doc_shown || return 0

  zrbmp_section "Google Cloud Platform Setup"
  
  zrbmp_subsection "Overview"
  zrbmp_normal "Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges."
  zrbmp_normal "The provisioner will automate the creation of operational service accounts and infrastructure configuration."
  
  zrbmp_subsection "Prerequisites"
  zrbmp_normal "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbmp_normal "- Email address not already associated with GCP"
  zrbmp_normal ""
  zrbmp_normal "---"
  
  zrbmp_section "Manual Provisioner Setup Procedure"
  
  zrbmp_normal "Recipe Bottle setup requires a manual bootstrap procedure to enable enough control"
  zrbmp_normal ""
  zrbmp_nc "Open a web browser to " "https://cloud.google.com/free"
  zrbmp_normal ""
  
  zrbmp_normal "1. **Establish Account**"
  zrbmp_nw "   1. Click " "\"Get started for free\""
  zrbmp_normal "   1. Sign in with Google account or create new"
  zrbmp_normal "   1. Provide:"
  zrbmp_normal "      - Country"
  zrbmp_nw "      - Organization type: " "Individual"
  zrbmp_normal "      - Credit card (verification only)"
  zrbmp_nw "   1. Accept terms → " "Start my free trial"
  zrbmp_normal "   1. Expect Google Cloud Console to open."
  
  zrbmp_normal "1. **Create New Project**"
  zrbmp_nw "   1. Top bar project dropdown → " "New Project"
  zrbmp_normal "   1. Configure:"
  zrbmp_nc "      - Project name: " "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}"
  zrbmp_nw "      - Leave organization as " "\"No organization\""
  zrbmp_nwn "   1. Create → Wait for notification " "\"Creating project...\"" " to complete"
  zrbmp_normal "   1. Select project from dropdown when ready"
  
  zrbmp_normal "1. **Create Provisioner Service Account**"
  zrbmp_normal "   1. Navigate to IAM & Admin section"
  zrbmp_nw "   1. Left sidebar → " "IAM & Admin → Service Accounts"
  zrbmp_nw "   1. If prompted about APIs, click " "\"Enable API\""
  zrbmp_nw "   1. Wait for " "\"Identity and Access Management (IAM) API\" to enable"
  
  zrbmp_normal "1. **Create the Provisioner**"
  zrbmp_nw "   1. Click " "\"+ CREATE SERVICE ACCOUNT\" at top"
  zrbmp_normal "   1. Service account details:"
  zrbmp_nc "      - Service account name: " "rbra-provisioner"
  zrbmp_nwn "      - Service account ID: (auto-fills as " "rbra-provisioner" ")"
  zrbmp_nc "      - Description: " "Temporary provisioner for infrastructure setup - DELETE AFTER USE"
  zrbmp_nw "   1. Click " "\"CREATE AND CONTINUE\""
  
  zrbmp_normal "1. Assign Project Owner Role:"
  
  zrbmp_critical "This grants complete project control. Delete immediately after setup."
  
  zrbmp_normal "Grant access section:"
  zrbmp_nw "1. Click " "\"Select a role\" dropdown"
  zrbmp_nc "2. In filter box, type: " "owner"
  zrbmp_nw "3. Select: " "Basic → Owner"
  zrbmp_nw "4. Click " "\"CONTINUE\""
  zrbmp_nw "5. Grant users access section: Skip (click " "\"DONE\")"
  zrbmp_normal ""
  zrbmp_nw "Service account list now shows " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  
  zrbmp_subsubsection "4. Generate Service Account Key"
  
  zrbmp_normal "From service accounts list:"
  zrbmp_nw "1. Click on " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  zrbmp_nw "2. Top tabs → " "KEYS"
  zrbmp_nw "3. Click " "\"ADD KEY\" → \"Create new key\""
  zrbmp_nw "4. Key type: " "JSON (should be selected)"
  zrbmp_nw "5. Click " "\"CREATE\""
  zrbmp_normal ""
  zrbmp_nw "Browser downloads: " "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-[random].json"
  zrbmp_normal ""
  zrbmp_nw "6. Click " "\"CLOSE\" on download confirmation"
  
  zrbmp_subsubsection "5. Configure Local Environment"
  
  zrbmp_normal "Open terminal ⟨LOCAL-SETUP⟩:"
  zrbmp_normal ""
  zrbmp_command "# Create secrets directory structure"
  zrbmp_command "mkdir -p ../station-files/secrets"
  zrbmp_command "cd ../station-files/secrets"
  zrbmp_command ""
  zrbmp_command "# Move downloaded key (adjust path to your Downloads folder)"
  zrbmp_command "mv ~/Downloads/${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-*.json rbra-provisioner-key.json"
  zrbmp_command ""
  zrbmp_command "# Verify key structure"
  zrbmp_command "jq -r '.type' rbra-provisioner-key.json"
  zrbmp_command "# Should output: service_account"
  zrbmp_command ""
  zrbmp_command "# Create RBRA environment file"
  zrbmp_command "cat > rbra-provisioner.env << 'EOF'"
  zrbmp_command "RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-provisioner-key.json"
  zrbmp_command "RBRA_TOKEN_LIFETIME_SEC=1800"
  zrbmp_command "EOF"
  zrbmp_command ""
  zrbmp_command "# Set restrictive permissions"
  zrbmp_command "chmod 600 rbra-provisioner-key.json"
  zrbmp_command "chmod 600 rbra-provisioner.env"
  
  zrbmp_normal ""
  zrbmp_normal "---"
  
  zrbmp_warning "Remember to delete the provisioner service account after infrastructure setup is complete!"
  
  zrbmp_normal "The provisioner environment file is now configured at:"
  zrbmp_nc "" "${RBRR_PROVISIONER_RBRA_FILE:-../station-files/secrets/rbra-provisioner.env}"
  zrbmp_normal ""
  
  bcu_success "Manual setup procedure displayed"
}

rbmp_show_teardown() {
  zrbmp_sentinel

  bcu_doc_brief "Display the procedure for removing the provisioner after setup"
  bcu_doc_shown || return 0

  zrbmp_section "Provisioner Teardown Procedure"
  
  zrbmp_critical "Execute this immediately after infrastructure setup is complete!"
  
  zrbmp_normal "1. **Delete Service Account Key from GCP Console**"
  zrbmp_nw "   1. Navigate to " "IAM & Admin → Service Accounts"
  zrbmp_nw "   2. Click on " "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  zrbmp_nw "   3. Go to " "KEYS tab"
  zrbmp_normal "   4. Find the key created earlier"
  zrbmp_nw "   5. Click the three dots menu → " "Delete"
  zrbmp_normal "   6. Confirm deletion"
  zrbmp_normal ""
  
  zrbmp_normal "2. **Delete Service Account**"
  zrbmp_normal "   1. Return to Service Accounts list"
  zrbmp_nw "   2. Check the box next to " "rbra-provisioner"
  zrbmp_nw "   3. Click " "DELETE at top"
  zrbmp_normal "   4. Type the confirmation text"
  zrbmp_nw "   5. Click " "DELETE"
  zrbmp_normal ""
  
  zrbmp_normal "3. **Remove Local Files**"
  zrbmp_command "# Remove provisioner credentials"
  zrbmp_command "cd ../station-files/secrets"
  zrbmp_command "shred -vuz rbra-provisioner-key.json"
  zrbmp_command "shred -vuz rbra-provisioner.env"
  zrbmp_normal ""
  
  zrbmp_normal "4. **Verify Removal**"
  zrbmp_normal "   - Check GCP Console shows no rbra-provisioner service account"
  zrbmp_normal "   - Verify local files are removed:"
  zrbmp_command "ls -la ../station-files/secrets/ | grep provisioner"
  zrbmp_normal "   (should return nothing)"
  zrbmp_normal ""
  
  bcu_success "Teardown procedure displayed"
}

