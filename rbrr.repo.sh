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

export RBRR_NEEDED_DISK_IMAGES="mow_x86_64_wsl mos_x86_64_qemu mos_aarch64_applehv"

export RBRR_CHOSEN_PODMAN_VERSION=5.5
export RBRR_CHOSEN_VMIMAGE_ORIGIN=quay.io/podman/machine-os-wsl   # Alt is quay.io/podman/machine-os

export RBRR_CHOSEN_IDENTITY=20250720-082409  # update date


# File containing user specific secrets for accessing the container registry.  Must have
# contents expressed as bash variables (i.e. no spaces around '=') as follows...
#
#     RBRG_USERNAME=yyyy
#     RBRG_PAT=ghp_zzzzzz
#
# ...where...
#
#     RBRG_USERNAME: GitHub username required for container registry (ghcr.io) login
#     RBRG_PAT: GitHub Personal Access Token used for both:
#              1. GitHub API authentication (for building/listing/deleting images)
#              2. Container registry authentication (for pulling images)
#              Generate this token at https://github.com/settings/tokens with scopes:
#              - read:packages, write:packages, delete:packages
#              - repo (for workflow dispatch)
export RBRR_GITHUB_PAT_ENV=../station-files/secrets/rbs-github.env


# eof
