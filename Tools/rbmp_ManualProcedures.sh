rbmp_show_setup() {
  zrbmp_sentinel
  
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

  # Step 0: Account Type Warning
  zrbmp_critic "This procedure is for PERSONAL Google accounts only."
  zrbmp_n      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbmp_n      "you must follow your IT/admin process to create projects, attach billing,"
  zrbmp_n      "and assign permissions — those steps are NOT covered here."
  zrbmp_e

  # Step 1: Establish Account
  zrbmp_s2     "1. Establish Account:"
  zrbmp_nc     "   Open a browser to: " "https://cloud.google.com/free"
  zrbmp_nw     "   1. Click -> " "Get started for free"
  zrbmp_n      "   2. Sign in with your Google account or create a new one"
  zrbmp_n      "   3. Provide:"
  zrbmp_n      "      - Country"
  zrbmp_nw     "      - Organization type: " "Individual"
  zrbmp_n      "      - Credit card (verification only)"
  zrbmp_nw     "   4. Accept terms → " "Start my free trial"
  zrbmp_n      "   5. Expect Google Cloud Console to open"
  zrbmp_nwn    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbmp_e

  # Step 2: Configure Project ID and Region
  local z_configure_pid_step="2. Configure Project ID and Region"
  zrbmp_s2     "${z_configure_pid_step}:"
  zrbmp_n      "   Before creating the project, choose a unique Project ID."
  zrbmp_nc     "   1. Edit your RBRR configuration file: " "${ZRBMP_RBRR_FILE}"
  zrbmp_n      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbmp_n      "      - Must be globally unique across all GCP"
  zrbmp_n      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbmp_n      "      - Cannot start/end with hyphen"
  zrbmp_n      "   3. Set RBRR_GCP_REGION based on your location (see project documentation)"
  zrbmp_n      "   4. Save the file before proceeding"
  zrbmp_e

  # Step 3: Create New Project
  zrbmp_s2     "3. Create New Project:"
  zrbmp_nc     "   Go directly to: " "https://console.cloud.google.com/"
  zrbmp_n      "   Sign in with the same Google account you just set up"
  zrbmp_nw     "   1. Left navigation → " "Manage resources"
  zrbmp_nw     "   2. Click → " "CREATE PROJECT"
  zrbmp_n      "   3. Configure:"
  zrbmp_nc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbmp_nw     "      - Organization: " "No organization"
  zrbmp_nw     "   4. Click " "CREATE"
  zrbmp_nwne   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbmp_nwn    "   6. Wait for notification → " "Creating project..." " to complete"
  zrbmp_n      "   7. Select project from dropdown when ready"
  zrbmp_e

  # Step 4: Navigate to Service Accounts
  zrbmp_s2     "4. Navigate to Service Accounts:"
  zrbmp_n      "   Ensure your new project is selected in the top dropdown"
  zrbmp_nw     "   1. Left sidebar → " "IAM & Admin → Service Accounts"
  zrbmp_nw     "   2. If prompted about APIs, click → " "Enable API"
  zrbmp_n      "      TODO: This step is brittle — enabling IAM API may happen automatically or be blocked by org policy."
  zrbmp_nw     "   3. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbmp_e

  # Step 5: Create the Provisioner
  zrbmp_s2     "5. Create the Provisioner:"
  zrbmp_nw     "   1. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbmp_n      "   2. Service account details:"
  zrbmp_nc     "      - Service account name: " "${ZRBMP_PROVISIONER_ROLE}"
  zrbmp_nwn    "      - Service account ID: (auto-fills as " "${ZRBMP_PROVISIONER_ROLE}" ")"
  zrbmp_nc     "      - Description: " "Temporary provisioner for infrastructure setup - DELETE AFTER USE"
  zrbmp_nw     "   3. Click → " "CREATE AND CONTINUE"
  zrbmp_e

  # Step 6: Assign Project Owner Role
  zrbmp_s2     "6. Assign Project Owner Role:"
  zrbmp_critic "This grants complete project control. Delete immediately after setup."
  zrbmp_n      "Grant access section:"
  zrbmp_nw     "   1. Click dropdown " "Select a role"
  zrbmp_nc     "   2. In filter box, type: " "owner"
  zrbmp_nw     "   3. Select: " "Basic → Owner"
  zrbmp_nw     "   4. Click → " "CONTINUE"
  zrbmp_nw     "   5. Grant users access section: Skip by clicking → " "DONE"
  zrbmp_e

  # Step 7: Generate Service Account Key
  zrbmp_s2     "7. Generate Service Account Key:"
  zrbmp_n      "From service accounts list:"
  zrbmp_nw     "   1. Click on " "${ZRBMP_PROVISIONER_ROLE}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"
  zrbmp_nw     "   2. Top tabs → " "KEYS"
  zrbmp_nwnw   "   3. Click " "ADD KEY" " → " "Create new key"
  zrbmp_nw     "   4. Key type: " "JSON (should be selected)"
  zrbmp_nw     "   5. Click " "CREATE"
  zrbmp_e
  zrbmp_nw     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbmp_nwn    "   6. Click " "CLOSE" " on download confirmation"
  zrbmp_e

  # Step 8: Configure Local Environment
  zrbmp_s2     "8. Configure Local Environment:"
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

