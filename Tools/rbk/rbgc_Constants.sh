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
  readonly RBGC_GLOBAL_PREFIX="rbwg"
  readonly RBGC_GLOBAL_TYPE_PAYOR="p"
  readonly RBGC_GLOBAL_TYPE_DEPOT="d"
  readonly RBGC_GLOBAL_TYPE_BUCKET="b"
  readonly RBGC_GLOBAL_TIMESTAMP_FORMAT="+%y%m%d%H%M%S"
  readonly RBGC_GLOBAL_TIMESTAMP_LEN=12
  readonly RBGC_GLOBAL_TIMESTAMP_REGEX="[0-9]{${RBGC_GLOBAL_TIMESTAMP_LEN}}"

  # Global resource validation patterns
  # Payor:  rbwg-p-YYMMDDHHMMSS  (timestamp survives — payor is installation-scoped, not depot-scoped)
  readonly RBGC_GLOBAL_PAYOR_REGEX="^${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-${RBGC_GLOBAL_TIMESTAMP_REGEX}$"

  # Basic Configuration
  readonly RBGC_ADMIN_ROLE="rbw-admin"
  readonly RBGC_PAYOR_ROLE="rbw-payor"
  readonly RBGC_PAYOR_APP_NAME="Recipe Bottle Payor"

  # Service Account Prefixes
  readonly RBGC_MASON_PREFIX="mason"

  # Timeouts
  readonly RBGC_MAX_CONSISTENCY_SEC=90
  readonly RBGC_EVENTUAL_CONSISTENCY_SEC=3
  readonly RBGC_SA_KEY_CREATE_RETRY_MAX=7
  readonly RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC=10

  # URL Roots & Well-known Endpoints
  readonly RBGC_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"
  readonly RBGC_API_ROOT_IAM="https://iam.googleapis.com"
  readonly RBGC_API_ROOT_CRM="https://cloudresourcemanager.googleapis.com"
  readonly RBGC_API_ROOT_SERVICEUSAGE="https://serviceusage.googleapis.com"
  readonly RBGC_API_ROOT_ARTIFACTREGISTRY="https://artifactregistry.googleapis.com"
  readonly RBGC_API_ROOT_CLOUDBUILD="https://cloudbuild.googleapis.com"
  readonly RBGC_API_ROOT_CLOUDBILLING="https://cloudbilling.googleapis.com"
  readonly RBGC_API_ROOT_STORAGE="https://storage.googleapis.com"
  readonly RBGC_API_ROOT_SECRETMANAGER="https://secretmanager.googleapis.com"
  readonly RBGC_CONSOLE_URL="https://console.cloud.google.com/"
  readonly RBGC_SIGNUP_URL="https://cloud.google.com/free"

  # OAuth Scopes
  readonly RBGC_SCOPE_CLOUD_PLATFORM="https://www.googleapis.com/auth/cloud-platform"

  # Service Usage Service Identifiers
  readonly RBGC_SERVICE_IAM="iam.googleapis.com"
  readonly RBGC_SERVICE_CRM="cloudresourcemanager.googleapis.com"
  readonly RBGC_SERVICE_ARTIFACTREGISTRY="artifactregistry.googleapis.com"

  # Email/Domain Assembly
  readonly RBGC_SA_EMAIL_DOMAIN="iam.gserviceaccount.com"

  # API Version Paths
  readonly RBGC_IAM_V1="/v1"
  readonly RBGC_CRM_V1="/v1"
  readonly RBGC_CRM_V3="/v3"
  readonly RBGC_SERVICEUSAGE_V1="/v1"
  readonly RBGC_SERVICEUSAGE_V1BETA1="/v1beta1"
  readonly RBGC_ARTIFACTREGISTRY_V1="/v1"
  readonly RBGC_CLOUDBUILD_V1="/v1"
  readonly RBGC_CLOUDBILLING_V1="/v1"
  readonly RBGC_STORAGE_JSON_V1="/storage/v1"
  readonly RBGC_STORAGE_JSON_UPLOAD="/upload/storage/v1"
  readonly RBGC_SECRETMANAGER_V1="/v1"

  # REST Path Fragments
  readonly RBGC_PATH_PROJECTS="/projects"
  readonly RBGC_PATH_LOCATIONS="/locations"
  readonly RBGC_PATH_REPOSITORIES="/repositories"
  readonly RBGC_PATH_SERVICE_ACCOUNTS="/serviceAccounts"
  readonly RBGC_PATH_KEYS="/keys"

  # REST Operation Suffixes
  readonly RBGC_CRM_GET_IAM_POLICY_SUFFIX=":getIamPolicy"
  readonly RBGC_CRM_SET_IAM_POLICY_SUFFIX=":setIamPolicy"
  readonly RBGC_SERVICEUSAGE_ENABLE_SUFFIX=":enable"
  readonly RBGC_SERVICEUSAGE_PATH_SERVICES="/services"

  # Operation Prefixes
  readonly RBGC_OP_PREFIX_GLOBAL="operations/"

  # Ark Artifact Basenames (₢A_AAK layout)
  # Each ark type is a plain basename sibling under <prefix>hallmarks/<hallmark>/.
  readonly RBGC_ARK_BASENAME_IMAGE="image"
  readonly RBGC_ARK_BASENAME_ABOUT="about"
  readonly RBGC_ARK_BASENAME_VOUCH="vouch"
  readonly RBGC_ARK_BASENAME_DIAGS="diags"
  readonly RBGC_ARK_BASENAME_ATTEST="attest"
  readonly RBGC_ARK_BASENAME_POUCH="pouch"

  # GAR Categorical Namespaces (₢A_AAK layout)
  # Top-level prefix-rooted namespaces under which hallmark/reliquary/enshrine
  # arks are stored as plain basename siblings. Consumed by rbgl_GarLayout.sh.
  readonly RBGC_GAR_CATEGORY_HALLMARKS="hallmarks"
  readonly RBGC_GAR_CATEGORY_RELIQUARIES="reliquaries"
  readonly RBGC_GAR_CATEGORY_ENSHRINES="enshrines"

  # Reliquary Tool Basenames (₢A_AAK layout)
  # Canonical tool names under <prefix>reliquaries/<date>/. Authoritative
  # manifest lives in rbgji/rbgji01-inscribe-mirror.sh.
  readonly RBGC_RELIQUARY_TOOL_GCLOUD="gcloud"
  readonly RBGC_RELIQUARY_TOOL_DOCKER="docker"
  readonly RBGC_RELIQUARY_TOOL_ALPINE="alpine"
  readonly RBGC_RELIQUARY_TOOL_SYFT="syft"
  readonly RBGC_RELIQUARY_TOOL_BINFMT="binfmt"
  readonly RBGC_RELIQUARY_TOOL_SKOPEO="skopeo"

  # Fact-file filenames (written to BURD_OUTPUT_DIR by producers, read by tests)
  readonly RBF_FACT_HALLMARK="rbf_fact_hallmark"
  readonly RBF_FACT_BUILD_ID="rbf_fact_build_id"
  readonly RBF_FACT_GAR_ROOT="rbf_fact_gar_root"
  readonly RBF_FACT_ARK_STEM="rbf_fact_ark_stem"
  readonly RBF_FACT_ARK_YIELD="rbf_fact_ark_yield"
  readonly RBF_FACT_RELIQUARY="rbf_fact_reliquary"

  # Payor fact-file filenames (governor identifying values)
  readonly RBGP_FACT_GOVERNOR_SA_EMAIL="rbgp_fact_governor_sa_email"

  # Depot fact-file extension and lifecycle-state vocabulary.
  # rbgp_depot_list emits one fact file per known depot named "<moniker>.depot"
  # with content equal to one of the state values below.
  readonly RBGP_FACT_EXT_DEPOT="depot"
  readonly RBGP_FACT_EXT_DEPOT_PROJECT="depot-project"
  readonly RBGP_DEPOT_STATE_COMPLETE="COMPLETE"
  readonly RBGP_DEPOT_STATE_BROKEN="BROKEN"
  readonly RBGP_DEPOT_STATE_DELETE_REQUESTED="DELETE_REQUESTED"

  # Artifact Registry (GAR) Composition
  readonly RBGC_GAR_HOST_SUFFIX="-docker.pkg.dev"


  # Canonical Role IDs
  readonly RBGC_ROLE_ARTIFACTREGISTRY_READER="roles/artifactregistry.reader"
  readonly RBGC_ROLE_ARTIFACTREGISTRY_WRITER="roles/artifactregistry.writer"
  readonly RBGC_ROLE_ARTIFACTREGISTRY_ADMIN="roles/artifactregistry.admin"
  readonly RBGC_ROLE_CONTAINERANALYSIS_OCCURRENCES_VIEWER="roles/containeranalysis.occurrences.viewer"
  readonly RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR="roles/cloudbuild.builds.editor"

  # Common API Base Paths (project-independent)
  readonly RBGC_API_BASE_GCS="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}"

  # Cloud Resource Manager - Liens API
  readonly RBGC_API_CRM_LIST_LIENS="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"
  readonly RBGC_API_CRM_DELETE_LIEN="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"

  # Google Cloud Storage (GCS) APIs (project-independent)
  readonly RBGC_API_GCS_BUCKETS="${RBGC_API_BASE_GCS}/b"

  readonly RBGC_BUILD_RUNNER_PLATFORM="linux/amd64"


  # Worker pool infrastructure (dual pools: tether + airgap)
  readonly RBGC_POOL_SUFFIX_TETHER="-tether"
  readonly RBGC_POOL_SUFFIX_AIRGAP="-airgap"
  readonly RBGC_WORKER_POOL_SUFFIX="-pool"
  readonly RBGC_PATH_WORKER_POOLS="/workerPools"

  readonly ZRBGC_KINDLED=1
}

zrbgc_sentinel() {
  test "${ZRBGC_KINDLED:-}" = "1" || buc_die "Module rbgc not kindled - call zrbgc_kindle first"
}

# eof

