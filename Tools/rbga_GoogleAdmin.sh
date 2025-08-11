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
# Recipe Bottle Google Admin - Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGA_SOURCED:-}" || bcu_die "Module rbga multiply sourced - check sourcing hierarchy"
ZRBGA_SOURCED=1

######################################################################
# Internal Functions (zrbga_*)

zrbga_kindle() {
  test -z "${ZRBGA_KINDLED:-}" || bcu_die "Module rbga already kindled"

  # Check terminal color support and adjust accordingly
  if [ -t 1 ] && [ "${TERM}" != "dumb" ] && [ "${NO_COLOR:-}" = "" ]; then
    # Terminal supports colors
    ZRBGA_R="\033[0m"         # Reset
    ZRBGA_S="\033[1;37m"      # Section (bright white)
    ZRBGA_C="\033[36m"        # Command (cyan)
    ZRBGA_W="\033[35m"        # Website (magenta)
    ZRBGA_WN="\033[1;33m"     # Warning (bright yellow)
    ZRBGA_CR="\033[1;31m"     # Critical (bright red)
  else
    # No color support or colors disabled
    ZRBGA_R=""
    ZRBGA_S=""
    ZRBGA_C=""
    ZRBGA_W=""
    ZRBGA_WN=""
    ZRBGA_CR=""
  fi

  ZRBGA_ADMIN_ROLE="rbga-admin"
  ZRBGA_RBRR_FILE="./rbrr_RecipeBottleRegimeRepo.sh"

  ZRBGA_GAR_READER_NAME="rbga-gar-reader"
  ZRBGA_GAR_READER_EMAIL="${ZRBGA_GAR_READER_NAME}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"

  ZRBGA_GCB_SUBMITTER_NAME="rbga-gcb-submitter"
  ZRBGA_GCB_SUBMITTER_EMAIL="${ZRBGA_GCB_SUBMITTER_NAME}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"

  ZRBGA_PREFIX="${BDU_TEMP_DIR}/rbga_"
  ZRBGA_LIST_RESPONSE="${ZRBGA_PREFIX}list_response.json"
  ZRBGA_LIST_CODE="${ZRBGA_PREFIX}list_code.txt"
  ZRBGA_CREATE_REQUEST="${ZRBGA_PREFIX}create_request.json"
  ZRBGA_CREATE_RESPONSE="${ZRBGA_PREFIX}create_response.json"
  ZRBGA_CREATE_CODE="${ZRBGA_PREFIX}create_code.txt"
  ZRBGA_DELETE_RESPONSE="${ZRBGA_PREFIX}delete_response.json"
  ZRBGA_DELETE_CODE="${ZRBGA_PREFIX}delete_code.txt"

  ZRBGA_KINDLED=1
}

zrbga_sentinel() {
  test "${ZRBGA_KINDLED:-}" = "1" || bcu_die "Module rbga not kindled - call zrbga_kindle first"
}

zrbga_show() {
  zrbga_sentinel
  echo -e "${1:-}"
}

zrbga_s1() { zrbga_show "${ZRBGA_S}${1}${ZRBGA_R}"; }
zrbga_s2() { zrbga_show "${ZRBGA_S}${1}${ZRBGA_R}"; }
zrbga_s3() { zrbga_show "${ZRBGA_S}${1}${ZRBGA_R}"; }

zrbga_e()    { zrbga_show; }
zrbga_n()    { zrbga_show "${1}"; }
zrbga_nc()   { zrbga_show "${1}${ZRBGA_C}${2}${ZRBGA_R}"; }
zrbga_ncn()  { zrbga_show "${1}${ZRBGA_C}${2}${ZRBGA_R}${3}"; }
zrbga_nw()   { zrbga_show "${1}${ZRBGA_W}${2}${ZRBGA_R}"; }
zrbga_nwn()  { zrbga_show "${1}${ZRBGA_W}${2}${ZRBGA_R}${3}"; }
zrbga_nwne() { zrbga_show "${1}${ZRBGA_W}${2}${ZRBGA_R}${3}${ZRBGA_CR}${4}${ZRBGA_R}"; }
zrbga_nwnw() { zrbga_show "${1}${ZRBGA_W}${2}${ZRBGA_R}${3}${ZRBGA_W}${4}${ZRBGA_R}"; }

zrbga_ne()   { zrbga_show "${1}${ZRBGA_CR}${2}${ZRBGA_R}"; }

zrbga_cmd()     { zrbga_show "${ZRBGA_C}${1}${ZRBGA_R}"; }
zrbga_warning() { zrbga_show "\n${ZRBGA_WN}⚠️  WARNING: ${1}${ZRBGA_R}\n"; }
zrbga_critic()  { zrbga_show "\n${ZRBGA_CR}🔴 CRITICAL SECURITY WARNING: ${1}${ZRBGA_R}\n"; }

zrbga_get_token_capture() {
  zrbga_sentinel

  local z_token
  z_token=$(rbgo_get_token_capture "${RBRR_ADMIN_RBRA_FILE}") || return 1

  test -n "${z_token}" || return 1
  echo "${z_token}"
}

zrbga_extract_json_to_rbra() {
  zrbga_sentinel

  local z_json_path="$1"
  local z_rbra_path="$2"
  local z_lifetime_sec="$3"

  test -f "${z_json_path}" || bcu_die "Service account JSON not found: ${z_json_path}"

  bcu_info "Extracting service account credentials from JSON"

  # Extract fields
  local z_client_email
  z_client_email=$(jq -r '.client_email' "${z_json_path}") || bcu_die "Failed to extract client_email"
  test -n "${z_client_email}" || bcu_die "Empty client_email in JSON"
  test "${z_client_email}" != "null" || bcu_die "Null client_email in JSON"

  local z_private_key
  z_private_key=$(jq -r '.private_key' "${z_json_path}") || bcu_die "Failed to extract private_key"
  test -n "${z_private_key}" || bcu_die "Empty private_key in JSON"
  test "${z_private_key}" != "null" || bcu_die "Null private_key in JSON"

  local z_project_id
  z_project_id=$(jq -r '.project_id' "${z_json_path}") || bcu_die "Failed to extract project_id"
  test -n "${z_project_id}" || bcu_die "Empty project_id in JSON"
  test "${z_project_id}" != "null" || bcu_die "Null project_id in JSON"

  # Verify project matches
  test "${z_project_id}" = "${RBRR_GCP_PROJECT_ID}" || bcu_die "Project mismatch: JSON has '${z_project_id}', expected '${RBRR_GCP_PROJECT_ID}'"

  # Write RBRA file
  echo "RBRA_CLIENT_EMAIL=\"${z_client_email}\"" > "${z_rbra_path}"
  echo "RBRA_PRIVATE_KEY=\"${z_private_key}\"" >> "${z_rbra_path}"
  echo "RBRA_PROJECT_ID=\"${z_project_id}\"" >> "${z_rbra_path}"
  echo "RBRA_TOKEN_LIFETIME_SEC=${z_lifetime_sec}" >> "${z_rbra_path}"

  test -f "${z_rbra_path}" || bcu_die "Failed to write RBRA file: ${z_rbra_path}"

  bcu_warn "Consider deleting source JSON after verification: ${z_json_path}"
}

######################################################################
# External Functions (rbga_*)

rbga_show_setup() {
  zrbga_sentinel

  bcu_doc_brief "Display the manual GCP admin setup procedure"
  bcu_doc_shown || return 0

  zrbga_s1     "# Google Cloud Platform Setup"
  zrbga_s2     "## Overview"
  zrbga_n      "Bootstrap GCP infrastructure by creating an admin service account with Project Owner privileges."
  zrbga_n      "The admin account will manage operational service accounts and infrastructure configuration."
  zrbga_s2     "## Prerequisites"
  zrbga_n      "- Credit card for GCP account verification (won't be charged on free tier)"
  zrbga_n      "- Email address not already associated with GCP"
  zrbga_e
  zrbga_n      "---"
  zrbga_s1     "Manual Admin Setup Procedure"
  zrbga_n      "Recipe Bottle setup requires a manual bootstrap procedure to enable admin control"
  zrbga_e
  zrbga_nc     "Open a web browser to " "https://cloud.google.com/free"

  zrbga_critic "This procedure is for PERSONAL Google accounts only."
  zrbga_n      "If your account is managed by an ORGANIZATION (e.g., Google Workspace),"
  zrbga_n      "you must follow your IT/admin process to create projects, attach billing,"
  zrbga_n      "and assign permissions — those steps are NOT covered here."
  zrbga_e
  zrbga_s2     "1. Establish Account:"
  zrbga_nc     "   Open a browser to: " "https://cloud.google.com/free"
  zrbga_nw     "   1. Click -> " "Get started for free"
  zrbga_n      "   2. Sign in with your Google account or create a new one"
  zrbga_n      "   3. Provide:"
  zrbga_n      "      - Country"
  zrbga_nw     "      - Organization type: " "Individual"
  zrbga_n      "      - Credit card (verification only)"
  zrbga_nw     "   4. Accept terms → " "Start my free trial"
  zrbga_n      "   5. Expect Google Cloud Console to open"
  zrbga_nwn    "   6. You should see: " "Welcome, [Your Name]" " with a 'Set Up Foundation' button"
  zrbga_e
  local z_configure_pid_step="2. Configure Project ID and Region"
  zrbga_s2     "${z_configure_pid_step}:"
  zrbga_n      "   Before creating the project, choose a unique Project ID."
  zrbga_nc     "   1. Edit your RBRR configuration file: " "${ZRBGA_RBRR_FILE}"
  zrbga_n      "   2. Set RBRR_GCP_PROJECT_ID to a unique value:"
  zrbga_n      "      - Must be globally unique across all GCP"
  zrbga_n      "      - 6-30 characters, lowercase letters, numbers, hyphens"
  zrbga_n      "      - Cannot start/end with hyphen"
  zrbga_n      "   3. Set RBRR_GCP_REGION based on your location (see project documentation)"
  zrbga_n      "   4. Save the file before proceeding"
  zrbga_e
  zrbga_s2     "3. Create New Project:"
  zrbga_nc     "   Go directly to: " "https://console.cloud.google.com/"
  zrbga_n      "   Sign in with the same Google account you just set up"
  zrbga_n      "   1. Open the Google Cloud Console main menu:"
  zrbga_nwn    "      - Click the " "☰" " hamburger menu in the top-left corner"
  zrbga_nw     "      - Scroll down to " "IAM & Admin"
  zrbga_nw     "      - Click → " "Manage resources"
  zrbga_n      "        (Alternatively, type 'manage resources' in the top search bar and press Enter)"
  zrbga_nw     "   2. On the Manage resources page, click → " "CREATE PROJECT"
  zrbga_n      "   3. Configure:"
  zrbga_nc     "      - Project name: " "${RBRR_GCP_PROJECT_ID}"
  zrbga_nw     "      - Organization: " "No organization"
  zrbga_nw     "   4. Click " "CREATE"
  zrbga_nwne   "   5. If " "The project ID is already taken" " : " "FAIL THIS STEP and redo with different project-ID: ${z_configure_pid_step}"
  zrbga_nwn    "   6. Wait for notification → " "Creating project..." " to complete"
  zrbga_n      "   7. Select project from dropdown when ready"
  zrbga_e
  zrbga_s2     "4. Navigate to Service Accounts:"
  zrbga_nwn    "   Ensure your new project is selected in the top dropdown (button with hovertext " "Open project picker (Ctrl O)" ")"
  zrbga_nwnw   "   1. Left sidebar → " "IAM & Admin" " → " "Service Accounts"
  zrbga_nw     "   2. If prompted about APIs, click → " "Enable API"
  zrbga_n      "      TODO: This step is brittle — enabling IAM API may happen automatically or be blocked by org policy."
  zrbga_nw     "   3. Wait for " "Identity and Access Management (IAM) API to enable"
  zrbga_e
  zrbga_s2     "5. Create the Admin Account:"
  zrbga_nw     "   1. At top, click " "+ CREATE SERVICE ACCOUNT"
  zrbga_n      "   2. Service account details:"
  zrbga_nc     "      - Service account name: " "${ZRBGA_ADMIN_ROLE}"
  zrbga_nwn    "      - Service account ID: (auto-fills as " "${ZRBGA_ADMIN_ROLE}" ")"
  zrbga_nc     "      - Description: " "Admin account for infrastructure management"
  zrbga_nw     "   3. Click → " "CREATE AND CONTINUE"
  zrbga_e
  zrbga_s2     "6. Assign Project Owner Role:"
  zrbga_n      "Grant access section:"
  zrbga_nw     "   1. Click dropdown " "Select a role"
  zrbga_nc     "   2. In filter box, type: " "owner"
  zrbga_nwnw   "   3. Select: " "Basic" " → " "Owner"
  zrbga_nw     "   4. Click → " "CONTINUE"
  zrbga_nwnw   "   5. Skip " "Principals with access" " by clicking → " "DONE"
  zrbga_e
  zrbga_s2     "7. Generate Service Account Key:"
  zrbga_n      "From service accounts list:"
  zrbga_nw     "   1. Click on text of " "${ZRBGA_ADMIN_ROLE}@${RBRR_GCP_PROJECT_ID}.iam.gserviceaccount.com"
  zrbga_nw     "   2. Top tabs → " "KEYS"
  zrbga_nwnw   "   3. Click " "ADD KEY" " → " "Create new key"
  zrbga_nwn    "   4. Key type: " "JSON" " (should be selected)"
  zrbga_nw     "   5. Click " "CREATE"
  zrbga_e
  zrbga_nw     "Browser downloads: " "${RBRR_GCP_PROJECT_ID}-[random].json"
  zrbga_nwn    "   6. Click " "CLOSE" " on download confirmation"
  zrbga_e
  zrbga_s2     "8. Configure Local Environment:"
  zrbga_cmd    "# Create secrets directory structure"
  zrbga_cmd    "mkdir -p ../station-files/secrets"
  zrbga_cmd    "cd ../station-files/secrets"
  zrbga_e
  zrbga_cmd    "# Move downloaded key (adjust path to your Downloads folder)"
  zrbga_cmd    "mv ~/Downloads/${RBRR_GCP_PROJECT_ID}-*.json ${ZRBGA_ADMIN_ROLE}-key.json"
  zrbga_e
  zrbga_cmd    "# Verify key structure"
  zrbga_cmd    "jq -r '.type' ${ZRBGA_ADMIN_ROLE}-key.json"
  zrbga_cmd    "# Should output: service_account"
  zrbga_e
  zrbga_cmd    "# Create RBRA environment file"
  zrbga_cmd    "echo 'RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/${ZRBGA_ADMIN_ROLE}-key.json' > ${ZRBGA_ADMIN_ROLE}.env"
  zrbga_cmd    "echo 'RBRA_TOKEN_LIFETIME_SEC=1800' >> ${ZRBGA_ADMIN_ROLE}.env"
  zrbga_e
  zrbga_cmd    "# Set restrictive permissions"
  zrbga_cmd    "chmod 600 ${ZRBGA_ADMIN_ROLE}-key.json"
  zrbga_cmd    "chmod 600 ${ZRBGA_ADMIN_ROLE}.env"
  zrbga_e
  zrbga_n      "The admin environment file is now configured at:"
  zrbga_nc     "" "${RBRR_ADMIN_RBRA_FILE:-../station-files/secrets/${ZRBGA_ADMIN_ROLE}.env}"
  zrbga_e

  bcu_success "Manual setup procedure displayed"
}

rbga_convert_admin_json() {
  zrbga_sentinel

  local z_json_path="${1:-}"

  bcu_doc_brief "Convert admin service account JSON to RBRA format"
  bcu_doc_param "json_path" "Path to downloaded admin JSON file"
  bcu_doc_shown || return 0

  test -n "${z_json_path}" || bcu_die "JSON path required"

  bcu_step "Converting admin JSON to RBRA format"

  zrbga_extract_json_to_rbra \
    "${z_json_path}" \
    "${RBRR_ADMIN_RBRA_FILE}" \
    "1800"

  bcu_success "Admin RBRA file created: ${RBRR_ADMIN_RBRA_FILE}"
}

rbga_list_service_accounts() {
  zrbga_sentinel

  bcu_doc_brief "List all service accounts in the project"
  bcu_doc_shown || return 0

  bcu_step "Listing service accounts in project: ${RBRR_GCP_PROJECT_ID}"

  # Get OAuth token from admin
  local z_token
  z_token=$(zrbga_get_token_capture) || bcu_die "Failed to get admin token"

  # List service accounts via REST API
  curl -s -X GET \
    "https://iam.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/serviceAccounts" \
    -H "Authorization: Bearer ${z_token}" \
    -o "${ZRBGA_LIST_RESPONSE}" \
    -w "%{http_code}" > "${ZRBGA_LIST_CODE}" 2>/dev/null

  local z_http_code=$(<"${ZRBGA_LIST_CODE}")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" != "200"; then
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_LIST_RESPONSE}") || z_error="Parse error"
    bcu_die "Failed to list service accounts (HTTP ${z_http_code}): ${z_error}"
  fi

  # Check if accounts exist
  local z_count
  z_count=$(jq -r '.accounts | length' "${ZRBGA_LIST_RESPONSE}") || bcu_die "Failed to parse response"

  if test "${z_count}" = "0" || test "${z_count}" = "null"; then
    bcu_info "No service accounts found in project"
    return 0
  fi

  # Display accounts
  bcu_info "Found ${z_count} service account(s):"

  jq -r '.accounts[] | "  \(.email) - \(.displayName // "(no display name)")"' "${ZRBGA_LIST_RESPONSE}" || bcu_die "Failed to format accounts"

  bcu_success "Service account listing completed"
}

rbga_create_gar_reader() {
  zrbga_sentinel

  bcu_doc_brief "Create GAR reader service account"
  bcu_doc_shown || return 0

  bcu_step "Creating GAR reader service account"

  # Get OAuth token from admin
  local z_token
  z_token=$(zrbga_get_token_capture) || bcu_die "Failed to get admin token"

  # Create request JSON using jq
  jq -n \
    --arg account_id "${ZRBGA_GAR_READER_NAME}" \
    --arg display_name "Recipe Bottle GAR Reader" \
    --arg description "Read-only access to Google Artifact Registry" \
    '{
      accountId: $account_id,
      serviceAccount: {
        displayName: $display_name,
        description: $description
      }
    }' > "${ZRBGA_CREATE_REQUEST}" || bcu_die "Failed to create request JSON"

  # Create service account
  curl -s -X POST \
    "https://iam.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/serviceAccounts" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Content-Type: application/json" \
    -d @"${ZRBGA_CREATE_REQUEST}" \
    -o "${ZRBGA_CREATE_RESPONSE}" \
    -w "%{http_code}" > "${ZRBGA_CREATE_CODE}" 2>/dev/null

  local z_http_code=$(<"${ZRBGA_CREATE_CODE}")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" = "200"; then
    local z_email
    z_email=$(jq -r '.email' "${ZRBGA_CREATE_RESPONSE}") || bcu_die "Failed to extract email"
    bcu_info "Created service account: ${z_email}"
  elif test "${z_http_code}" = "409"; then
    bcu_warn "GAR reader already exists: ${ZRBGA_GAR_READER_EMAIL}"
  else
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_CREATE_RESPONSE}") || z_error="Parse error"
    bcu_die "Failed to create GAR reader (HTTP ${z_http_code}): ${z_error}"
  fi

  bcu_success "GAR reader service account ready"
}

rbga_create_gcb_submitter() {
  zrbga_sentinel

  bcu_doc_brief "Create GCB submitter service account"
  bcu_doc_shown || return 0

  bcu_step "Creating GCB submitter service account"

  # Get OAuth token from admin
  local z_token
  z_token=$(zrbga_get_token_capture) || bcu_die "Failed to get admin token"

  # Create request JSON using jq
  jq -n \
    --arg account_id "${ZRBGA_GCB_SUBMITTER_NAME}" \
    --arg display_name "Recipe Bottle GCB Submitter" \
    --arg description "Submit builds to Google Cloud Build" \
    '{
      accountId: $account_id,
      serviceAccount: {
        displayName: $display_name,
        description: $description
      }
    }' > "${ZRBGA_CREATE_REQUEST}" || bcu_die "Failed to create request JSON"

  # Create service account
  curl -s -X POST \
    "https://iam.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/serviceAccounts" \
    -H "Authorization: Bearer ${z_token}" \
    -H "Content-Type: application/json" \
    -d @"${ZRBGA_CREATE_REQUEST}" \
    -o "${ZRBGA_CREATE_RESPONSE}" \
    -w "%{http_code}" > "${ZRBGA_CREATE_CODE}" 2>/dev/null

  local z_http_code=$(<"${ZRBGA_CREATE_CODE}")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" = "200"; then
    local z_email
    z_email=$(jq -r '.email' "${ZRBGA_CREATE_RESPONSE}") || bcu_die "Failed to extract email"
    bcu_info "Created service account: ${z_email}"
  elif test "${z_http_code}" = "409"; then
    bcu_warn "GCB submitter already exists: ${ZRBGA_GCB_SUBMITTER_EMAIL}"
  else
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_CREATE_RESPONSE}") || z_error="Parse error"
    bcu_die "Failed to create GCB submitter (HTTP ${z_http_code}): ${z_error}"
  fi

  bcu_success "GCB submitter service account ready"
}

rbga_delete_service_account() {
  zrbga_sentinel

  local z_sa_email="${1:-}"

  bcu_doc_brief "Delete a service account"
  bcu_doc_param "email" "Email address of the service account to delete"
  bcu_doc_shown || return 0

  test -n "${z_sa_email}" || bcu_die "Service account email required"

  bcu_step "Deleting service account: ${z_sa_email}"

  # Get OAuth token from admin
  local z_token
  z_token=$(zrbga_get_token_capture) || bcu_die "Failed to get admin token"

  # Delete via REST API
  curl -s -X DELETE \
    "https://iam.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}/serviceAccounts/${z_sa_email}" \
    -H "Authorization: Bearer ${z_token}" \
    -o "${ZRBGA_DELETE_RESPONSE}" \
    -w "%{http_code}" > "${ZRBGA_DELETE_CODE}" 2>/dev/null

  local z_http_code=$(<"${ZRBGA_DELETE_CODE}")
  test -n "${z_http_code}" || bcu_die "Failed to read HTTP code"

  if test "${z_http_code}" = "200" || test "${z_http_code}" = "204"; then
    bcu_info "Deleted service account: ${z_sa_email}"
  elif test "${z_http_code}" = "404"; then
    bcu_warn "Service account not found (already deleted): ${z_sa_email}"
  else
    local z_error
    z_error=$(jq -r '.error.message // "Unknown error"' "${ZRBGA_DELETE_RESPONSE}") || z_error="Parse error"
    bcu_die "Failed to delete (HTTP ${z_http_code}): ${z_error}"
  fi

  bcu_success "Delete operation completed"
}

rbga_cleanup_subordinates() {
  zrbga_sentinel

  bcu_doc_brief "Delete all subordinate service accounts (GAR reader and GCB submitter)"
  bcu_doc_shown || return 0

  bcu_step "Cleaning up subordinate service accounts"

  rbga_delete_service_account "${ZRBGA_GAR_READER_EMAIL}"
  rbga_delete_service_account "${ZRBGA_GCB_SUBMITTER_EMAIL}"

  bcu_success "Subordinate cleanup completed"
}



# eof

