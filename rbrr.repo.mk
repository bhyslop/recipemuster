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

RBRR_REGISTRY_OWNER      = bhyslop
RBRR_REGISTRY_NAME       = recipemuster
RBRR_BUILD_ARCHITECTURES = linux/amd64
RBRR_HISTORY_DIR         = RBM-history
RBRR_NAMEPLATE_PATH      = RBM-nameplates
RBRR_DNS_SERVER          = 8.8.8.8
RBRR_MACHINE_NAME        = podman-machine-rbw
RBRR_MACHINE_IMAGE       = Ubuntu-22.04

# File containing user specific secrets for accessing the container registry.  Must define:
#
# RBV_USERNAME: GitHub username required for container registry (ghcr.io) login
# RBV_PAT: GitHub Personal Access Token used for both:
#          1. GitHub API authentication (for building/listing/deleting images)
#          2. Container registry authentication (for pulling images)
#          Generate this token at https://github.com/settings/tokens with scopes:
#          - read:packages, write:packages, delete:packages
#          - repo (for workflow dispatch)
RBRR_GITHUB_PAT_ENV = ../secrets/github-ghcr-play.env


# eof
