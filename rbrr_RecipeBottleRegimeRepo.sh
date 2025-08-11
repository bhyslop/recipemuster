#!/bin/bash
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
# Recipe Bottle Repository Configuration - Base Values

export RBRR_REGISTRY_OWNER=bhyslop
export RBRR_REGISTRY_NAME=recipemuster
export RBRR_BUILD_ARCHITECTURES=linux/amd64
export RBRR_HISTORY_DIR=RBM-history
export RBRR_NAMEPLATE_PATH=RBM-nameplates
export RBRR_DNS_SERVER=8.8.8.8

export RBRR_IGNITE_MACHINE_NAME=rbw-vm-ignite
export RBRR_DEPLOY_MACHINE_NAME=rbw-vm-deploy
export RBRR_CRANE_TAR_GZ=https://github.com/google/go-containerregistry/releases/download/v0.20.3/go-containerregistry_Linux_x86_64.tar.gz

export RBRR_MANIFEST_PLATFORMS="mow_x86_64_wsl mow_aarch64_wsl" # "mos_x86_64_qemu mos_aarch64_applehv"

export RBRR_CHOSEN_PODMAN_VERSION=5.5
export RBRR_CHOSEN_VMIMAGE_ORIGIN=quay.io/podman/machine-os-wsl   # Alt is quay.io/podman/machine-os

export RBRR_CHOSEN_IDENTITY=20250723-092042  # mow_x86_64_wsl mow_aarch64_wsl


# Google Cloud Platform configuration (shared by all GCP services)
export RBRR_GCP_PROJECT_ID=brm-recipemuster-proj
export RBRR_GCP_REGION=us-central1  # PRoject doesn't have a region but our services do

# Google Artifact Registry settings
export RBRR_GAR_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
export RBRR_GAR_LOCATION="${RBRR_GCP_REGION}"
export RBRR_GAR_REPOSITORY=brm_recipemuster_gar

# Google Cloud Build settings  
export RBRR_GCB_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
export RBRR_GCB_REGION="${RBRR_GCP_REGION}"
export RBRR_GCB_MACHINE_TYPE=e2-highcpu-8
export RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout
export RBRR_GCB_STAGING_BUCKET=gs://your-build-staging-bucket


# Service Account Configuration - RBRA (Recipe Bottle Regime Auth) Format
#
# RBRA files are bash-sourceable environment files containing extracted 
# Google service account credentials. No JSON parsing required at runtime.
#
# Required RBRA environment variables:
#   RBRA_CLIENT_EMAIL       - Service account email (e.g., sa-name@project.iam.gserviceaccount.com)
#   RBRA_PRIVATE_KEY        - RSA private key in PEM format (includes newlines as \n)
#   RBRA_PROJECT_ID         - GCP project ID for context
#   RBRA_TOKEN_LIFETIME_SEC - OAuth token lifetime in seconds (300-3600)
#
# Creation process:
#   1. Download service account JSON from GCP Console
#   2. Extract fields: jq -r '.client_email' key.json
#   3. Create RBRA file with proper escaping for RBRA_PRIVATE_KEY
#   4. Delete original JSON file
#   5. chmod 600 the RBRA file
#
# Security notes:
#   - RBRA files contain raw credentials - protect like passwords
#   - Never commit RBRA files to version control
#   - Use shortest reasonable TOKEN_LIFETIME_SEC for each role
#
# Service accounts by role:
#
# 1. GAR READER - Pull/list container images
export RBRR_GAR_RBRA_FILE=../station-files/secrets/rbra-gar-reader.env
#    Permissions: roles/artifactregistry.reader
#    Token lifetime: 300 seconds (quick pulls)
#
# 2. GCB SUBMITTER - Submit Cloud Build jobs  
export RBRR_GCB_RBRA_FILE=../station-files/secrets/rbra-gcb-submitter.env
#    Permissions: roles/cloudbuild.builds.editor + storage.admin on staging
#    Token lifetime: 600 seconds (build submission)
#
# 3. ADMIN - PROJECT OWNER
export RBRR_ADMIN_RBRA_FILE=../station-files/secrets/rbra-admin.env
#    Permissions: roles/owner (FULL PROJECT CONTROL)
#    Token lifetime: 1800 seconds (complex setup operations)


# eof
