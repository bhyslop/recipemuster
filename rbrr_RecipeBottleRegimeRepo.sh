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

RBRR_REGISTRY_OWNER=bhyslop
RBRR_REGISTRY_NAME=recipemuster
RBRR_DNS_SERVER=8.8.8.8

RBRR_IGNITE_MACHINE_NAME=rbw-vm-ignite
RBRR_DEPLOY_MACHINE_NAME=rbw-vm-deploy
RBRR_CRANE_TAR_GZ=https://github.com/google/go-containerregistry/releases/download/v0.20.3/go-containerregistry_Linux_x86_64.tar.gz

RBRR_MANIFEST_PLATFORMS="mow_x86_64_wsl mow_aarch64_wsl" # "mos_x86_64_qemu mos_aarch64_applehv"

RBRR_CHOSEN_PODMAN_VERSION=5.5
RBRR_CHOSEN_VMIMAGE_ORIGIN=quay.io/podman/machine-os-wsl   # Alt is quay.io/podman/machine-os

RBRR_CHOSEN_IDENTITY=20250723-092042  # mow_x86_64_wsl mow_aarch64_wsl

# Reference directory containing specs for all vessels: relative to project root
RBRR_VESSEL_DIR=rbev-vessels

# Google Cloud Platform configuration (shared by all GCP services)
RBRR_DEPOT_PROJECT_ID=rbwg-d-proto-251230080456
RBRR_GCP_REGION=us-central1  # Project doesn't have a region but our services do

# Google Artifact Registry settings
RBRR_GAR_REPOSITORY=rbw-proto-repository

# Google Cloud Build settings  
RBRR_GCB_MACHINE_TYPE=E2_HIGHCPU_8          # Google Cloud Build machine type (enum form as in web API, not gcloud CLI)
RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout

########################################################################
# Google Cloud Build tool image pins (digest-pinned, ~spring 2025)
# These are only used inside GCB build steps (_RBGY_* substitutions).

# gcrane: registry utility (~Apr 2025)
RBRR_GCB_GCRANE_IMAGE_REF="gcr.io/go-containerregistry/gcrane@sha256:b6f6b744e7b5db9f50a85d3c7c0a7f5e04f04d1ad26d872d23eec92cb3dc5025"

# oras: OCI artifact/referrer client (~May 2025)
RBRR_GCB_ORAS_IMAGE_REF="ghcr.io/oras-project/oras@sha256:61b7765b4c2847d734e1d80f37c63dbfb11494ff0f40a32ab2d0c7e61028b5b1"


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
# Security notes:
#   - RBRA files contain raw credentials - protect like passwords
#   - Never commit RBRA files to version control
#   - Use shortest reasonable TOKEN_LIFETIME_SEC for each role
#
# Service accounts by role:
#
# 1. Pull/list container images
RBRR_RETRIEVER_RBRA_FILE=../station-files/secrets/rbra-retriever.env
#    Permissions: roles/artifactregistry.reader
#    Token lifetime: 300 seconds (quick pulls)
#
# 2. Submit Cloud Build image jobs, delete images
RBRR_DIRECTOR_RBRA_FILE=../station-files/secrets/rbra-director.env
#    Permissions: roles/cloudbuild.builds.editor + storage.admin on staging
#    Token lifetime: 600 seconds (build submission)
#
# 3. GOVERNOR
RBRR_GOVERNOR_RBRA_FILE=../station-files/secrets/rbra-governor.env
#    Permissions: roles/owner (FULL PROJECT CONTROL)
#    Token lifetime: 1800 seconds (complex setup operations)


# eof
