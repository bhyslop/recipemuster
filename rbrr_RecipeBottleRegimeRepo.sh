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

# Google Cloud Service Account Configuration
#
# Three distinct roles with least-privilege access:
#
# 1. REGISTRY READER (rbcr operations - pull/list/inspect)
#    - Used by: Developers pulling images, CI/CD systems
#    - Permissions: roles/artifactregistry.reader on GAR repository
#    - Token lifetime: 300 seconds (5 minutes)
#
# 2. BUILD SUBMITTER (rbgc operations - submit builds)  
#    - Used by: Release engineers, authorized build triggers
#    - Permissions: roles/cloudbuild.builds.editor + roles/storage.admin on staging bucket
#    - Token lifetime: 600 seconds (10 minutes for upload + submit)
#
# 3. BUILD RUNTIME (Cloud Build service account - NOT configured here)
#    - Used by: Cloud Build itself during execution
#    - Permissions: roles/artifactregistry.writer on GAR + source access
#    - Managed by: Google Cloud IAM, not local secrets
#    - Configure via: gcloud projects add-iam-policy-binding
#
# Path to GAR reader secrets file containing:
#   RBRG_GAR_SERVICE_ACCOUNT_KEY=/path/to/gar-reader-key.json
export RBRR_GAR_SERVICE_ENV=../station-files/secrets/rbs-gar-reader.env

# Path to Cloud Build submitter secrets file containing:
#   RBRG_GCB_SERVICE_ACCOUNT_KEY=/path/to/gcb-submitter-key.json
#   RBRG_GCB_STAGING_BUCKET=gs://your-build-staging-bucket
export RBRR_GCB_SERVICE_ENV=../station-files/secrets/rbs-gcb-submitter.env

# Token lifetime in seconds (300-3600 allowed by Google)
export RBRR_TOKEN_LIFETIME_SECONDS=300

# Cloud Build configuration (no secrets - uses default SA at runtime)
export RBRR_GCB_PROJECT_ID=${RBRR_GAR_PROJECT_ID}  # Often same project
export RBRR_GCB_REGION=us-central1                 # Build execution region
export RBRR_GCB_MACHINE_TYPE=e2-highcpu-8          # Build machine type
export RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout

# eof
