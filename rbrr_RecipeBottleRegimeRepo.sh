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
RBRR_BUILD_ARCHITECTURES=linux/amd64
RBRR_HISTORY_DIR=RBM-history
RBRR_NAMEPLATE_PATH=RBM-nameplates
RBRR_DNS_SERVER=8.8.8.8

RBRR_IGNITE_MACHINE_NAME=rbw-vm-ignite
RBRR_DEPLOY_MACHINE_NAME=rbw-vm-deploy
RBRR_CRANE_TAR_GZ=https://github.com/google/go-containerregistry/releases/download/v0.20.3/go-containerregistry_Linux_x86_64.tar.gz

RBRR_MANIFEST_PLATFORMS="mow_x86_64_wsl mow_aarch64_wsl" # "mos_x86_64_qemu mos_aarch64_applehv"

RBRR_CHOSEN_PODMAN_VERSION=5.5
RBRR_CHOSEN_VMIMAGE_ORIGIN=quay.io/podman/machine-os-wsl   # Alt is quay.io/podman/machine-os

RBRR_CHOSEN_IDENTITY=20250723-092042  # mow_x86_64_wsl mow_aarch64_wsl


# Google Cloud Platform configuration (shared by all GCP services)
RBRR_GCP_PROJECT_ID=brm-recipemuster-proj
RBRR_GCP_REGION=us-central1  # PRoject doesn't have a region but our services do

# Google Artifact Registry settings
RBRR_GAR_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
RBRR_GAR_LOCATION="${RBRR_GCP_REGION}"
RBRR_GAR_REPOSITORY=brm_recipemuster_gar

# Google Cloud Build settings  
RBRR_GCB_PROJECT_ID="${RBRR_GCP_PROJECT_ID}"
RBRR_GCB_REGION="${RBRR_GCP_REGION}"
RBRR_GCB_MACHINE_TYPE=e2-highcpu-8
RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout
RBRR_GCB_STAGING_BUCKET=gs://your-build-staging-bucket

# Toolchain pins for post-build utilities (policy: third-party only AFTER push)
# Rationale:
# - jq: choose 1.8.0 to incorporate CVE-2024-23337 fix (<=1.7.1 vulnerable).
# - syft: pick a ~6 month-old train (v1.20.0, 2025-02-22) to avoid "brand new"
#         while staying within the current audited release cadence.
# - binfmt: disabled by default; enable only if cross-arch emulation is required.
#
# Sources:
# - jq security & releases: CVE-2024-23337; jq 1.8.0; official GHCR package.
# - syft release cadence: community announcements around v1.20.0/v1.24.0.
# - binfmt release line: 2025-03-03.
###############################################################################

# --- jq (post-build JSON assembly) ---
# Prefer GHCR official image maintained by jqlang; pin by manifest-list digest.
# NOTE: This example digest is a starting point; verify once and rotate as needed.
RBRR_JQ_IMAGE_REF="ghcr.io/jqlang/jq@sha256:4f34c6d23f4b1372ac789752cc955dc67c2ae177eb1b5860b75cdc5091ce6f91"

# --- syft (post-build SBOM) ---
# Choose a conservative tag (v1.20.0 @ 2025-02-22). Resolve to a manifest-list
# digest once at pin time and record here. If you prefer a slightly newer base,
# consider v1.24.0 (2025-05-14). Replace TAG or DIGEST below with your final pick.
# Example tag (temporary until you resolve a digest):
RBRR_SYFT_IMAGE_REF="docker.io/anchore/syft:v1.20.0"
# After resolving with gcrane: (example placeholder; replace with your digest)
# RBRR_SYFT_IMAGE_REF="docker.io/anchore/syft@sha256:<RESOLVED_MANIFEST_DIGEST>"

# --- binfmt (pre-build emulation; keep disabled by default per policy) ---
# Only set when you explicitly allow non-Google containers *before* image creation.
# Populate with a known-good digest from a released cut (>=3 months old).
RBRR_BINFMT_IMAGE_REF="docker.io/tonistiigi/binfmt@sha256:<PIN_IF_ENABLED>"
RBRR_ENABLE_BINFMT="1"   # 0 = disabled (policy default), 1 = enable when required


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
# 3. PROJECT OWNER
RBRR_ADMIN_RBRA_FILE=../station-files/secrets/rbra-admin.env
#    Permissions: roles/owner (FULL PROJECT CONTROL)
#    Token lifetime: 1800 seconds (complex setup operations)



# eof
