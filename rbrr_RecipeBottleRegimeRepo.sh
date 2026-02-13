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
RBRR_GCB_MACHINE_TYPE=UNSPECIFIED            # Google Cloud Build machine type (2 vCPU default; fits 5 concurrent in 10-CPU quota)
RBRR_GCB_TIMEOUT=1200s                      # 20 minute timeout

########################################################################
# Google Cloud Build tool image pins (digest-pinned, ~spring 2025)
# These are only used inside GCB build steps (_RBGY_* substitutions).

# gcrane: registry utility (~Apr 2025)
RBRR_GCB_GCRANE_IMAGE_REF="gcr.io/go-containerregistry/gcrane@sha256:b34b7a92b50cb3017af7ae84916c7ca87e04c1766e639fbec6f0656a21d77470"

# oras: OCI artifact/referrer client (~May 2025)
RBRR_GCB_ORAS_IMAGE_REF="ghcr.io/oras-project/oras@sha256:a3ce6b38d4c510ea9fdc0449b942ea44fb790f157e79b5e7e30b1e7460fe5579"

# gcloud: GCP CLI builder (~Feb 2026)
RBRR_GCB_GCLOUD_IMAGE_REF="gcr.io/cloud-builders/gcloud@sha256:54bcf51d601ff370171bba523099424765c6f2aadabbc30bab2e2779978da6cd"

# docker: Docker/buildx builder (~Feb 2026)
RBRR_GCB_DOCKER_IMAGE_REF="gcr.io/cloud-builders/docker@sha256:80e36415eeda32a4e2fda4c551f6cf7a450afcaa2e91bf4eafaabf942cf69bcc"

# skopeo: OCI registry copy tool (~Feb 2026)
RBRR_GCB_SKOPEO_IMAGE_REF="quay.io/skopeo/stable@sha256:5e778799da518c1d49bc837b9e1760c7d48a1ea9ac8659412651adc4e7219126"

# alpine: minimal Linux for scripting steps (~Feb 2026)
RBRR_GCB_ALPINE_IMAGE_REF="docker.io/library/alpine@sha256:59855d3dceb3ae53991193bd03301e082b2a7faa56a514b03527ae0ec2ce3a95"

# syft: SBOM generator (~Feb 2026)
RBRR_GCB_SYFT_IMAGE_REF="docker.io/anchore/syft@sha256:d0b19da0182bc17145379ccb3434b3f7eae540676a2b5e6bdae67152f1fae892"

# binfmt: QEMU cross-platform registration (~Feb 2026)
RBRR_GCB_BINFMT_IMAGE_REF="docker.io/tonistiigi/binfmt@sha256:cc1e14d6380b2ede7d32669a9986c3b7550624f4a7bc0b5b649e5072598eecf7"


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
