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
# Recipe Bottle Google Constants - Implementation (no printf required)

set -euo pipefail

# Multiple inclusion detection
# (Module state remains ZRBGC_* per BCG; external constants use RBGC_*)
test -z "${ZRBGC_SOURCED:-}" || bcu_die "Module rbgc multiply sourced - check sourcing hierarchy"
ZRBGC_SOURCED=1

######################################################################
# Internal Functions (zrbgc_*)

zrbgc_kindle() {
  test -z "${ZRBGC_KINDLED:-}" || bcu_die "Module rbgc already kindled"

  RBGC_ADMIN_ROLE="rbga-admin"

  # Service-specific aliases
  RBGC_GAR_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
  RBGC_GAR_LOCATION="${RBRR_GCP_REGION}"
  RBGC_GCB_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
  RBGC_GCB_REGION="${RBRR_GCP_REGION}"

  # Google Cloud Storage bucket
  RBGC_GCS_BUCKET="${RBRR_GCP_PROJECT_ID}-artifacts"

  # ------------------------------------------------------------------
  # URL roots & well-known endpoints (fully expanded)
  # ------------------------------------------------------------------
  RBGC_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"
  RBGC_API_ROOT_IAM="https://iam.googleapis.com"
  RBGC_API_ROOT_CRM="https://cloudresourcemanager.googleapis.com"
  RBGC_API_ROOT_SERVICEUSAGE="https://serviceusage.googleapis.com"
  RBGC_API_ROOT_ARTIFACTREGISTRY="https://artifactregistry.googleapis.com"
  RBGC_CONSOLE_URL="https://console.cloud.google.com/"
  RBGC_SIGNUP_URL="https://cloud.google.com/free"

  RBGC_API_ROOT_CLOUDBUILD="https://cloudbuild.googleapis.com"
  RBGC_CLOUDBUILD_V1="/v1"

  # ------------------------------------------------------------------
  # OAuth scopes
  # ------------------------------------------------------------------
  RBGC_SCOPE_CLOUD_PLATFORM="https://www.googleapis.com/auth/cloud-platform"

  # ------------------------------------------------------------------
  # Service Usage service identifiers
  # ------------------------------------------------------------------
  RBGC_SERVICE_IAM="iam.googleapis.com"
  RBGC_SERVICE_CRM="cloudresourcemanager.googleapis.com"
  RBGC_SERVICE_ARTIFACTREGISTRY="artifactregistry.googleapis.com"

  # ------------------------------------------------------------------
  # IAM email/domain assembly (no templates)
  # Usage: "${account_id}@${project_id}.${RBGC_SA_EMAIL_DOMAIN}"
  # ------------------------------------------------------------------
  RBGC_SA_EMAIL_DOMAIN="iam.gserviceaccount.com"

  # ------------------------------------------------------------------
  # REST path fragments (compose with simple concatenation, no printf)
  # Examples:
  #   List SAs:
  #     "${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}${RBGC_PATH_PROJECTS}/${project}${RBGC_PATH_SERVICE_ACCOUNTS}"
  #   Get/Set IAM Policy (CRM):
  #     "${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${project}${RBGC_CRM_GET_IAM_POLICY_SUFFIX}"
  # ------------------------------------------------------------------
  RBGC_IAM_V1="/v1"
  RBGC_CRM_V1="/v1"
  RBGC_SERVICEUSAGE_V1="/v1"
  RBGC_ARTIFACTREGISTRY_V1="/v1"

  RBGC_PATH_PROJECTS="/projects"
  RBGC_PATH_LOCATIONS="/locations"
  RBGC_PATH_REPOSITORIES="/repositories"
  RBGC_PATH_SERVICE_ACCOUNTS="/serviceAccounts"
  RBGC_PATH_KEYS="/keys"

  RBGC_CRM_GET_IAM_POLICY_SUFFIX=":getIamPolicy"
  RBGC_CRM_SET_IAM_POLICY_SUFFIX=":setIamPolicy"

  RBGC_SERVICEUSAGE_PATH_SERVICES="/services"
  RBGC_SERVICEUSAGE_ENABLE_SUFFIX=":enable"

  # ------------------------------------------------------------------
  # Artifact Registry (GAR) composition bits (concatenate at call-sites)
  # Example HOST:  "${location}${RBGC_GAR_HOST_SUFFIX}"
  # Example URI:   "${location}${RBGC_GAR_HOST_SUFFIX}/${project}/${repo}/${image}:${tag}"
  # ------------------------------------------------------------------
  RBGC_GAR_HOST_SUFFIX="-docker.pkg.dev"

  # ------------------------------------------------------------------
  # Canonical role IDs
  # ------------------------------------------------------------------
  RBGC_ROLE_ARTIFACTREGISTRY_READER="roles/artifactregistry.reader"
  RBGC_ROLE_ARTIFACTREGISTRY_WRITER="roles/artifactregistry.writer"
  RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR="roles/cloudbuild.builds.editor"
  RBGC_ROLE_ARTIFACTREGISTRY_ADMIN="roles/artifactregistry.admin"

  # Rolled up constants
  RBGC_API_SERVICE_ACCOUNTS="${RBGC_API_ROOT_IAM}${RBGC_IAM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_PATH_SERVICE_ACCOUNTS}"
  RBGC_SA_EMAIL_FULL="${RBRR_GCP_PROJECT_ID}.${RBGC_SA_EMAIL_DOMAIN}"
  RBGC_API_CRM_GET_IAM_POLICY="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_CRM_GET_IAM_POLICY_SUFFIX}"
  RBGC_API_CRM_SET_IAM_POLICY="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_CRM_SET_IAM_POLICY_SUFFIX}"
  RBGC_API_SU_ENABLE_IAM="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_IAM}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_ENABLE_CRM="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_CRM}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_ENABLE_GAR="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_ARTIFACTREGISTRY}${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_VERIFY_IAM="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_IAM}"
  RBGC_API_SU_VERIFY_CRM="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_CRM}"
  RBGC_API_SU_VERIFY_GAR="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/${RBGC_SERVICE_ARTIFACTREGISTRY}"
  RBGC_API_CRM_GET_PROJECT="https://cloudresourcemanager.googleapis.com/v1/projects/${RBRR_GCP_PROJECT_ID}"
  RBGC_API_SU_ENABLE_BUILD="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/cloudbuild.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_ENABLE_ANALYSIS="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/containeranalysis.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_ENABLE_STORAGE="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/storage.googleapis.com${RBGC_SERVICEUSAGE_ENABLE_SUFFIX}"
  RBGC_API_SU_VERIFY_BUILD="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/cloudbuild.googleapis.com"
  RBGC_API_SU_VERIFY_ANALYSIS="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/containeranalysis.googleapis.com"
  RBGC_API_SU_VERIFY_STORAGE="${RBGC_API_ROOT_SERVICEUSAGE}${RBGC_SERVICEUSAGE_V1}${RBGC_PATH_PROJECTS}/${RBRR_GCP_PROJECT_ID}${RBGC_SERVICEUSAGE_PATH_SERVICES}/storage.googleapis.com"

  # GCS bucket aspects
  RBGC_API_ROOT_STORAGE="https://storage.googleapis.com"
  RBGC_STORAGE_JSON_V1="/storage/v1"
  RBGC_STORAGE_JSON_UPLOAD="/upload/storage/v1"

  # GCS bucket operations
  RBGC_API_GCS_BUCKETS="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}/b"
  RBGC_API_GCS_BUCKET_CREATE="${RBGC_API_GCS_BUCKETS}?project=${RBRR_GCP_PROJECT_ID}"
  RBGC_API_GCS_BUCKET_OPS="${RBGC_API_GCS_BUCKETS}/${RBGC_GCS_BUCKET}"
  RBGC_API_GCS_BUCKET_OBJECTS="${RBGC_API_GCS_BUCKET_OPS}/o"
  RBGC_API_GCS_BUCKET_IAM="${RBGC_API_GCS_BUCKET_OPS}/iam"

  ZRBGC_KINDLED=1
}

zrbgc_sentinel() {
  test "${ZRBGC_KINDLED:-}" = "1" || bcu_die "Module rbgc not kindled - call zrbgc_kindle first"
}

# eof

