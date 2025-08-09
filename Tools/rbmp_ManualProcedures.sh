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

  # Source RBL module to get RBRR file location
  zrbl_kindle
  bvu_file_exists "${RBL_RBRR_FILE}"
  source "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR regime file"

  # Define ANSI color codes
  ZRBMP_COLOR_RESET="\033[0m"
  ZRBMP_COLOR_SECTION="\033[1;37m"   # Bright white for sections
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

zrbmp_section() {
  zrbmp_sentinel
  local z_title="${1}"
  echo -e "${ZRBMP_COLOR_SECTION}## ${z_title}${ZRBMP_COLOR_RESET}"
  echo
}

zrbmp_subsection() {
  zrbmp_sentinel
  local z_title="${1}"
  echo -e "${ZRBMP_COLOR_SECTION}### ${z_title}${ZRBMP_COLOR_RESET}"
  echo
}

zrbmp_command() {
  zrbmp_sentinel
  local z_cmd="${1}"
  echo -e "   ${ZRBMP_COLOR_COMMAND}${z_cmd}${ZRBMP_COLOR_RESET}"
}

zrbmp_website() {
  zrbmp_sentinel
  local z_text="${1}"
  echo -e "   ${ZRBMP_COLOR_WEBSITE}${z_text}${ZRBMP_COLOR_RESET}"
}

zrbmp_warning() {
  zrbmp_sentinel
  local z_text="${1}"
  echo -e "${ZRBMP_COLOR_WARNING}⚠️  WARNING: ${z_text}${ZRBMP_COLOR_RESET}"
  echo
}

zrbmp_critical() {
  zrbmp_sentinel
  local z_text="${1}"
  echo -e "${ZRBMP_COLOR_CRITICAL}🔴 CRITICAL SECURITY WARNING: ${z_text}${ZRBMP_COLOR_RESET}"
  echo
}

######################################################################
# External Functions (rbmp_*)

rbmp_show_setup() {
  zrbmp_sentinel

  bcu_doc_brief "Display the manual GCP provisioner setup procedure"
  bcu_doc_shown || return 0

  # Header
  zrbmp_section "Google Cloud Platform Setup"

  echo "Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges."
  echo "The provisioner will automate the creation of operational service accounts and infrastructure configuration."
  echo

  zrbmp_section "Prerequisites"
  echo "- Credit card for GCP account verification (won't be charged on free tier)"
  echo "- Email address not already associated with GCP"
  echo
  echo "---"
  echo

  zrbmp_section "Manual Provisioner Setup Procedure"
  echo
  echo "Recipe Bottle setup requires a manual bootstrap procedure to enable enough control"
  echo
  echo "Open a web browser to:"
  zrbmp_command "https://cloud.google.com/free"
  echo

  echo "1. **Establish Account**"
  echo "   1. Click"
  zrbmp_website "Get started for free"
  echo "   2. Sign in with Google account or create new"
  echo "   3. Provide:"
  echo "      - Country"
  echo "      - Organization type:"
  zrbmp_website "Individual"
  echo "      - Credit card (verification only)"
  echo "   4. Accept terms →"
  zrbmp_website "Start my free trial"
  echo "   5. Expect Google Cloud Console to open."
  echo

  echo "2. **Create New Project**"
  echo "   1. Top bar project dropdown →"
  zrbmp_website "New Project"
  echo "   2. Configure:"
  echo "      - Project name:"
  zrbmp_command "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}"
  echo "      - Leave organization as"
  zrbmp_website "No organization"
  echo "   3."
  zrbmp_website "Create"
  echo "      → Wait for notification"
  zrbmp_website "Creating project..."
  echo "      to complete"
  echo "   4. Select project from dropdown when ready"
  echo

  echo "3. **Create Provisioner Service Account**"
  echo "   1. Navigate to IAM & Admin section"
  echo "   2. Left sidebar →"
  zrbmp_website "IAM & Admin"
  echo "      →"
  zrbmp_website "Service Accounts"
  echo "   3. If prompted about APIs, click"
  zrbmp_website "Enable API"
  echo "   4. Wait for"
  zrbmp_website "Identity and Access Management (IAM) API"
  echo "      to enable"
  echo

  echo "4. **Create the Provisioner**"
  echo "   1. Click"
  zrbmp_website "+ CREATE SERVICE ACCOUNT"
  echo "      at top"
  echo "   2. Service account details:"
  echo "      - Service account name:"
  zrbmp_command "rbra-provisioner"
  echo "      - Service account ID: (auto-fills as"
  zrbmp_website "rbra-provisioner"
  echo "      )"
  echo "      - Description:"
  zrbmp_command "Temporary provisioner for infrastructure setup - DELETE AFTER USE"
  echo "   3. Click"
  zrbmp_website "CREATE AND CONTINUE"
  echo

  echo "5. **Assign Project Owner Role:**"
  echo
  zrbmp_critical "This grants complete project control. Delete immediately after setup."
  echo
  echo "   Grant access section:"
  echo "   1. Click"
  zrbmp_website "Select a role"
  echo "      dropdown"
  echo "   2. In filter box, type:"
  zrbmp_command "owner"
  echo "   3. Select:"
  zrbmp_website "Basic → Owner"
  echo "   4. Click"
  zrbmp_website "CONTINUE"
  echo "   5. Grant users access section: Skip (click"
  zrbmp_website "DONE"
  echo "      )"
  echo
  echo "   Service account list now shows:"
  zrbmp_website "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  echo

  zrbmp_subsection "6. Generate Service Account Key"
  echo
  echo "From service accounts list:"
  echo "   1. Click on"
  zrbmp_website "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  echo "   2. Top tabs →"
  zrbmp_website "KEYS"
  echo "   3. Click"
  zrbmp_website "ADD KEY"
  echo "      →"
  zrbmp_website "Create new key"
  echo "   4. Key type:"
  zrbmp_website "JSON"
  echo "      (should be selected)"
  echo "   5. Click"
  zrbmp_website "CREATE"
  echo
  echo "   Browser downloads:"
  zrbmp_website "${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-[random].json"
  echo
  echo "   6. Click"
  zrbmp_website "CLOSE"
  echo "      on download confirmation"
  echo

  zrbmp_subsection "7. Configure Local Environment"
  echo
  echo "Open terminal [LOCAL-SETUP]:"
  echo
  echo -e "${ZRBMP_COLOR_COMMAND}# Create secrets directory structure"
  echo "mkdir -p ../station-files/secrets"
  echo "cd ../station-files/secrets"
  echo
  echo "# Move downloaded key (adjust path to your Downloads folder)"
  echo "mv ~/Downloads/${RBRR_GAR_PROJECT_ID:-recipemuster-prod}-*.json rbra-provisioner-key.json"
  echo
  echo "# Verify key structure"
  echo "jq -r '.type' rbra-provisioner-key.json"
  echo "# Should output: service_account"
  echo
  echo "# Create RBRA environment file"
  echo "cat > rbra-provisioner.env << 'EOF'"
  echo "RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-provisioner-key.json"
  echo "RBRA_TOKEN_LIFETIME_SEC=1800"
  echo "EOF"
  echo
  echo "# Set restrictive permissions"
  echo "chmod 600 rbra-provisioner-key.json"
  echo -e "chmod 600 rbra-provisioner.env${ZRBMP_COLOR_RESET}"
  echo

  echo "---"
  echo
  zrbmp_warning "Remember to delete the provisioner service account after infrastructure setup is complete!"
  echo
  echo "The provisioner environment file is now configured at:"
  zrbmp_command "${RBRR_PROVISIONER_RBRA_FILE:-../station-files/secrets/rbra-provisioner.env}"
  echo

  bcu_success "Manual setup procedure displayed"
}

rbmp_show_teardown() {
  zrbmp_sentinel

  bcu_doc_brief "Display the procedure for removing the provisioner after setup"
  bcu_doc_shown || return 0

  zrbmp_section "Provisioner Teardown Procedure"

  zrbmp_critical "Execute this immediately after infrastructure setup is complete!"

  echo "1. **Delete Service Account Key from GCP Console**"
  echo "   1. Navigate to"
  zrbmp_website "IAM & Admin → Service Accounts"
  echo "   2. Click on"
  zrbmp_website "rbra-provisioner@${RBRR_GAR_PROJECT_ID:-recipemuster-prod}.iam.gserviceaccount.com"
  echo "   3. Go to"
  zrbmp_website "KEYS"
  echo "      tab"
  echo "   4. Find the key created earlier"
  echo "   5. Click the three dots menu →"
  zrbmp_website "Delete"
  echo "   6. Confirm deletion"
  echo

  echo "2. **Delete Service Account**"
  echo "   1. Return to Service Accounts list"
  echo "   2. Check the box next to"
  zrbmp_website "rbra-provisioner"
  echo "   3. Click"
  zrbmp_website "DELETE"
  echo "      at top"
  echo "   4. Type the confirmation text"
  echo "   5. Click"
  zrbmp_website "DELETE"
  echo

  echo "3. **Remove Local Files**"
  echo
  zrbmp_command "# Remove provisioner credentials"
  zrbmp_command "cd ../station-files/secrets"
  zrbmp_command "shred -vuz rbra-provisioner-key.json"
  zrbmp_command "shred -vuz rbra-provisioner.env"
  echo

  echo "4. **Verify Removal**"
  echo "   - Check GCP Console shows no rbra-provisioner service account"
  echo "   - Verify local files are removed:"
  echo
  zrbmp_command "ls -la ../station-files/secrets/ | grep provisioner"
  echo "   (should return nothing)"
  echo

  bcu_success "Teardown procedure displayed"
}
