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
# Recipe Bottle GCP Constants - Implementation (no printf required)

set -euo pipefail

# Multiple inclusion detection
# (Module state remains ZRBGC_* per BCG; external constants use RBGC_*)
test -z "${ZRBGC_SOURCED:-}" || buc_die "Module rbgc multiply sourced - check sourcing hierarchy"
ZRBGC_SOURCED=1

######################################################################
# Internal Functions (zrbgc_*)

zrbgc_kindle() {
  test -z "${ZRBGC_KINDLED:-}" || buc_die "Module rbgc already kindled"

  # Global Resource Naming (Google Cloud global namespace)
  # These resources compete in globally-unique namespaces across all of GCP
  # Pattern: {prefix}-{type}-{name}-{timestamp} where timestamp is YYMMDDHHMMSS
  RBGC_GLOBAL_PREFIX="rbwg"
  RBGC_GLOBAL_TYPE_PAYOR="p"
  RBGC_GLOBAL_TYPE_DEPOT="d"
  RBGC_GLOBAL_TYPE_BUCKET="b"
  RBGC_GLOBAL_TIMESTAMP_FORMAT="+%y%m%d%H%M%S"
  RBGC_GLOBAL_TIMESTAMP_LEN=12
  RBGC_GLOBAL_TIMESTAMP_REGEX="[0-9]{${RBGC_GLOBAL_TIMESTAMP_LEN}}"
  RBGC_GLOBAL_DEPOT_NAME_MAX=10

  # Global resource validation patterns
  # Payor:  rbwg-p-YYMMDDHHMMSS
  # Depot:  rbwg-d-[name]-YYMMDDHHMMSS
  # Bucket: rbwg-b-[name]-YYMMDDHHMMSS
  RBGC_GLOBAL_PAYOR_REGEX="^${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-${RBGC_GLOBAL_TIMESTAMP_REGEX}$"
  RBGC_GLOBAL_DEPOT_REGEX="^${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_DEPOT}-[a-z0-9-]+-${RBGC_GLOBAL_TIMESTAMP_REGEX}$"
  RBGC_GLOBAL_BUCKET_REGEX="^${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_BUCKET}-[a-z0-9-]+-${RBGC_GLOBAL_TIMESTAMP_REGEX}$"

  # Basic Configuration
  RBGC_ADMIN_ROLE="rbw-admin"
  RBGC_PAYOR_ROLE="rbw-payor"
  RBGC_PAYOR_APP_NAME="Recipe Bottle Payor"

  # Service Account Prefixes
  RBGC_GOVERNOR_PREFIX="governor"
  RBGC_MASON_PREFIX="mason"
  RBGC_DIRECTOR_PREFIX="director"
  RBGC_RETRIEVER_PREFIX="retriever"

  # Timeouts
  RBGC_MAX_CONSISTENCY_SEC=90
  RBGC_EVENTUAL_CONSISTENCY_SEC=3

  # URL Roots & Well-known Endpoints
  RBGC_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"
  RBGC_API_ROOT_IAM="https://iam.googleapis.com"
  RBGC_API_ROOT_CRM="https://cloudresourcemanager.googleapis.com"
  RBGC_API_ROOT_SERVICEUSAGE="https://serviceusage.googleapis.com"
  RBGC_API_ROOT_ARTIFACTREGISTRY="https://artifactregistry.googleapis.com"
  RBGC_API_ROOT_CLOUDBUILD="https://cloudbuild.googleapis.com"
  RBGC_API_ROOT_CLOUDBILLING="https://cloudbilling.googleapis.com"
  RBGC_API_ROOT_STORAGE="https://storage.googleapis.com"
  RBGC_CONSOLE_URL="https://console.cloud.google.com/"
  RBGC_SIGNUP_URL="https://cloud.google.com/free"

  # OAuth Scopes
  RBGC_SCOPE_CLOUD_PLATFORM="https://www.googleapis.com/auth/cloud-platform"

  # Service Usage Service Identifiers
  RBGC_SERVICE_IAM="iam.googleapis.com"
  RBGC_SERVICE_CRM="cloudresourcemanager.googleapis.com"
  RBGC_SERVICE_ARTIFACTREGISTRY="artifactregistry.googleapis.com"

  # Email/Domain Assembly
  RBGC_SA_EMAIL_DOMAIN="iam.gserviceaccount.com"

  # API Version Paths
  RBGC_IAM_V1="/v1"
  RBGC_CRM_V1="/v1"
  RBGC_CRM_V3="/v3"
  RBGC_SERVICEUSAGE_V1="/v1"
  RBGC_SERVICEUSAGE_V1BETA1="/v1beta1"
  RBGC_ARTIFACTREGISTRY_V1="/v1"
  RBGC_CLOUDBUILD_V1="/v1"
  RBGC_CLOUDBILLING_V1="/v1"
  RBGC_STORAGE_JSON_V1="/storage/v1"
  RBGC_STORAGE_JSON_UPLOAD="/upload/storage/v1"

  # REST Path Fragments
  RBGC_PATH_PROJECTS="/projects"
  RBGC_PATH_LOCATIONS="/locations"
  RBGC_PATH_REPOSITORIES="/repositories"
  RBGC_PATH_SERVICE_ACCOUNTS="/serviceAccounts"
  RBGC_PATH_KEYS="/keys"

  # REST Operation Suffixes
  RBGC_CRM_GET_IAM_POLICY_SUFFIX=":getIamPolicy"
  RBGC_CRM_SET_IAM_POLICY_SUFFIX=":setIamPolicy"
  RBGC_SERVICEUSAGE_ENABLE_SUFFIX=":enable"
  RBGC_SERVICEUSAGE_PATH_SERVICES="/services"

  # Operation Prefixes
  RBGC_OP_PREFIX_GLOBAL="operations/"

  # Ark Artifact Suffixes
  RBGC_ARK_SUFFIX_IMAGE="-image"
  RBGC_ARK_SUFFIX_ABOUT="-about"

  # Artifact Registry (GAR) Composition
  RBGC_GAR_HOST_SUFFIX="-docker.pkg.dev"


  # Canonical Role IDs
  RBGC_ROLE_ARTIFACTREGISTRY_READER="roles/artifactregistry.reader"
  RBGC_ROLE_ARTIFACTREGISTRY_WRITER="roles/artifactregistry.writer"
  RBGC_ROLE_ARTIFACTREGISTRY_ADMIN="roles/artifactregistry.admin"
  RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR="roles/cloudbuild.builds.editor"

  # Common API Base Paths (project-independent)
  RBGC_API_BASE_GCS="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}"

  # Cloud Resource Manager - Liens API
  RBGC_API_CRM_LIST_LIENS="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"
  RBGC_API_CRM_DELETE_LIEN="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"

  # Google Cloud Storage (GCS) APIs (project-independent)
  RBGC_API_GCS_BUCKETS="${RBGC_API_BASE_GCS}/b"

  ZRBGC_KINDLED=1
}

zrbgc_sentinel() {
  test "${ZRBGC_KINDLED:-}" = "1" || buc_die "Module rbgc not kindled - call zrbgc_kindle first"
}

######################################################################
# External Functions (rbgc_*)

# GCB default pool machine type to vCPU mapping.
#
# Provenance (verified 2026-02-13):
#   Enum names from Cloud Build API BuildOptions.MachineType:
#     https://cloud.google.com/dotnet/docs/reference/Google.Cloud.CloudBuild.V1/latest/Google.Cloud.CloudBuild.V1.BuildOptions.Types.MachineType
#   That reference gives enum names and CPU counts for most types but
#   describes UNSPECIFIED only as "Standard machine type" without a
#   vCPU count.
#
#   The UNSPECIFIED=2 vCPU value was inferred from:
#     1. Google web search result stating default is e2-standard-2 (2 vCPU)
#     2. Empirical: with 10-CPU quota, two UNSPECIFIED builds started
#        concurrently (impossible if >=6 vCPU each)
#     3. Consistent with project's own RBRR comment and 5-concurrent
#        builds observed at 10-CPU quota
#   No single authoritative Google document was found that explicitly
#   maps UNSPECIFIED to a vCPU count for the default pool.
#
# If Google adds new machine types, this function will hard-fail on
# the unknown value, forcing an update here.
rbgc_gcb_machine_vcpus_capture() {
  zrbgc_sentinel
  local z_machine_type="$1"
  case "${z_machine_type}" in
    UNSPECIFIED)    echo 2  ;;
    E2_MEDIUM)      echo 1  ;;
    E2_HIGHCPU_8)   echo 8  ;;
    E2_HIGHCPU_32)  echo 32 ;;
    N1_HIGHCPU_8)   echo 8  ;;
    N1_HIGHCPU_32)  echo 32 ;;
    *) buc_die "Unknown GCB machine type: ${z_machine_type} — update rbgc_gcb_machine_vcpus_capture" ;;
  esac
}

# eof

