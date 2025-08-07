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


# Google Artifact Registry settings
export RBRR_GAR_PROJECT_ID=your-project-id
export RBRR_GAR_LOCATION=us-central1
export RBRR_GAR_REPOSITORY=recipemuster

# Google Cloud Build settings
export RBRR_GCB_PROJECT_ID=${RBRR_GAR_PROJECT_ID}  # Often same project
export RBRR_GCB_REGION=us-central1                 # Build execution region
export RBRR_GCB_MACHINE_TYPE=e2-highcpu-8          # Build machine type
export RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout
export RBRR_GCB_STAGING_BUCKET=gs://your-build-staging-bucket

# Service Account Configuration
#
# Uses RBRA (Recipe Bottle Regime Alphabet) for Google service account credentials.
# Each service has distinct permissions following least-privilege:
#
# 1. GAR READER - Pull/list/inspect container images
#    - Permissions: roles/artifactregistry.reader
#    - Token lifetime: 300 seconds (5 minutes)
#    - File contains: RBRA_SERVICE_ACCOUNT_KEY, RBRA_TOKEN_LIFETIME_SEC
#
# 2. GCB SUBMITTER - Submit builds to Cloud Build
#    - Permissions: roles/cloudbuild.builds.editor + storage.admin on staging
#    - Token lifetime: 600 seconds (10 minutes)
#    - File contains: RBRA_SERVICE_ACCOUNT_KEY, RBRA_TOKEN_LIFETIME_SEC
#
export RBRR_GAR_RBRA_FILE=../station-files/secrets/rbrs-gar.env
export RBRR_GCB_RBRA_FILE=../station-files/secrets/rbrs-gcb.env


# eof
