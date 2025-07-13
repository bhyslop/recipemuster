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
export RBRR_MACHINE_NAME=podman-machine-rbw

# Determined from default podman machine startup chatter
# 
# Reference:
#   Looking up Podman Machine image at quay.io/podman/machine-os-wsl:5.3 to create VM
#   Getting image source signatures
#   Copying blob sha256:6898117ca935bae6cbdf680d5b8bb27c0f9fbdfe8799e5fe8aae7b87c17728d3
#   Copying config sha256:44136fa355b3678a1146ad16f7e8649e94fb4fc21fe77e8310c060f61caaff8a
#   Writing manifest to image destination
#
#  https://quay.io/repository/podman/machine-os-wsl/manifest/sha256:da977f55af1f69b6e4655b5a8faccc47b40034b29740f2d50e2b4d33cc1a7e16

# Base image reference
# OUCH should I split the pure version part out?
export RBRR_VMDIST_TAG=quay.io/podman/machine-os-wsl:5.3

# Index manifest that lists available architectures (top level)
export RBRR_VMDIST_INDEX_SHA=0bc492bd4071e8a2d84246dd6e67977b69ac7d0729591e46c0e28df166e97f84

# Platform-specific manifest for x86_64 (what we use)
export RBRR_VMDIST_X86_SHA=0bc492bd4071e8a2d84246dd6e67977b69ac7d0729591e46c0e28df166e97f84

# Actual content blob SHA (the filesystem layer)
export RBRR_VMDIST_BLOB_SHA=6898117ca935bae6cbdf680d5b8bb27c0f9fbdfe8799e5fe8aae7b87c17728d3

# Version and arch of utility for copying betwixt container registries
export RBRR_VMDIST_CRANE=https://github.com/google/go-containerregistry/releases/download/v0.20.3/go-containerregistry_Linux_x86_64.tar.gz


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
