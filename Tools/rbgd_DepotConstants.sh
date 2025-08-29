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
# Recipe Bottle GCP Depot Constants - Project-dependent Implementation

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBGD_SOURCED:-}" || bcu_die "Module rbgd multiply sourced - check sourcing hierarchy"
ZRBGD_SOURCED=1

######################################################################
# Internal Functions (zrbgd_*)

zrbgd_kindle() {
  test -z "${ZRBGD_KINDLED:-}" || bcu_die "Module rbgd already kindled"

  # Source and validate RBRR (Recipe Bottle Regime Repository) file
  bcu_log_args 'Use RBL to locate and source RBRR file'
  zrbl_sentinel
  test -f "${RBL_RBRR_FILE}" || bcu_die "RBRR file not found: ${RBL_RBRR_FILE}"
  source  "${RBL_RBRR_FILE}" || bcu_die "Failed to source RBRR file"

  bcu_log_args 'Validate RBRR variables using validator'
  ZRBGD_SCRIPT_DIR="$(dirname "${BASH_SOURCE[0]}")"
  source "${ZRBGD_SCRIPT_DIR}/rbrr.validator.sh" || bcu_die "Failed to validate RBRR variables"

  # Depot-specific Constants (require RBRR variables)
  RBGD_GCS_BUCKET="${RBRR_GCP_PROJECT_ID}-artifacts"

  # Service-specific Aliases
  RBGD_GAR_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
  RBGD_GAR_LOCATION="${RBRR_GCP_REGION}"
  RBGD_GCB_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
  RBGD_GCB_REGION="${RBRR_GCP_REGION}"

  # Project-dependent API Paths
  RBGD_PROJECT_RESOURCE="${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}"

  # Common API Base Paths (hoisted for reuse)
  RBGD_API_BASE_SERVICEUSAGE="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}"
  RBGD_API_BASE_CRM_PROJECT="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}"
  RBGD_API_BASE_IAM_PROJECT="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}"

  # IAM Service Accounts
  RBGD_API_SERVICE_ACCOUNTS="${RBGD_API_BASE_IAM_PROJECT}${RBGC_PATH_SERVICE_ACCOUNTS}"
  RBGD_SA_EMAIL_FULL="${RBRR_GCP_PROJECT_ID}.${RBGC_SA_EMAIL_DOMAIN}"
  RBGD_MASON_EMAIL="${RBGC_MASON_NAME}@${RBGD_SA_EMAIL_FULL}"

  # Cloud Resource Manager (CRM) APIs
  RBGD_API_CRM_GET_IAM_POLICY="${RBGD_API_BASE_CRM_PROJECT}${RBGC_CRM_GET_IAM_POLICY_SUFFIX}"
  RBGD_API_CRM_SET_IAM_POLICY="${RBGD_API_BASE_CRM_PROJECT}${RBGC_CRM_SET_IAM_POLICY_SUFFIX}"
  RBGD_API_CRM_GET_PROJECT="${RBGD_API_BASE_CRM_PROJECT}"
  RBGD_API_CRM_DELETE_PROJECT="${RBGD_API_BASE_CRM_PROJECT}"
  RBGD_API_CRM_UNDELETE_PROJECT="${RBGD_API_BASE_CRM_PROJECT}:undelete"

  # Service Usage - API Enablement
  RBGD_API_SU_ENABLE_IAM="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_IAM}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGD_API_SU_ENABLE_CRM="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_CRM}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGD_API_SU_ENABLE_GAR="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_ARTIFACTREGISTRY}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGD_API_SU_ENABLE_BUILD="${RBGD_API_BASE_SERVICEUSAGE}/cloudbuild.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGD_API_SU_ENABLE_ANALYSIS="${RBGD_API_BASE_SERVICEUSAGE}/containeranalysis.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGD_API_SU_ENABLE_STORAGE="${RBGD_API_BASE_SERVICEUSAGE}/storage.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"

  # Service Usage - API Verification
  RBGD_API_SU_VERIFY_IAM="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_IAM}"
  RBGD_API_SU_VERIFY_CRM="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_CRM}"
  RBGD_API_SU_VERIFY_GAR="${RBGD_API_BASE_SERVICEUSAGE}/${RBGC_SERVICE_ARTIFACTREGISTRY}"
  RBGD_API_SU_VERIFY_BUILD="${RBGD_API_BASE_SERVICEUSAGE}/cloudbuild.googleapis.com"
  RBGD_API_SU_VERIFY_ANALYSIS="${RBGD_API_BASE_SERVICEUSAGE}/containeranalysis.googleapis.com"
  RBGD_API_SU_VERIFY_STORAGE="${RBGD_API_BASE_SERVICEUSAGE}/storage.googleapis.com"

  # Google Cloud Storage (GCS) APIs
  RBGD_API_GCS_BUCKET_CREATE="${RBGC_API_GCS_BUCKETS}?project=${RBRR_GCP_PROJECT_ID}"
  RBGD_API_GCS_BUCKET_OPS="${RBGC_API_GCS_BUCKETS}/${RBGD_GCS_BUCKET}"
  RBGD_API_GCS_BUCKET_OBJECTS="${RBGD_API_GCS_BUCKET_OPS}/o"
  RBGD_API_GCS_BUCKET_IAM="${RBGD_API_GCS_BUCKET_OPS}/iam"

  ZRBGD_KINDLED=1
}

zrbgd_sentinel() {
  test "${ZRBGD_KINDLED:-}" = "1" || bcu_die "Module rbgd not kindled - call zrbgd_kindle first"
}

# eof