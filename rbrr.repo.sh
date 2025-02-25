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
#
# BILINGUAL FILE: This file is designed to be compatible with both:
# 1. Shell scripts (sourced with `. ./rbrb.repo.sh` or `source ./rbrb.repo.sh`)
# 2. Makefiles (included with `include rbrb.repo.sh`)
#
# USAGE NOTES:
# - When used in a shell context, variables are exported to the environment
# - When included in makefiles, variables become make variables
# - The export syntax is intentionally compatible with both contexts
# - Comments must start at beginning of line for Make compatibility
# - No conditionals or complex shell logic to maintain Make compatibility
# - Modifications should maintain this dual compatibility
# - For GitHub Actions workflows, this file can be sourced directly

# Registry Configuration
export RBRR_REGISTRY_SERVER=ghcr.io
export RBRR_REGISTRY_OWNER=bhyslop
export RBRR_REGISTRY_NAME=recipemuster

# Build Configuration
export RBRR_BUILD_ARCHITECTURES=linux/amd64
export RBRR_HISTORY_DIR=RBM-history

# Path Configuration
export RBRR_NAMEPLATE_PATH=RBM-nameplates

# DNS Configuration
export RBRR_DNS_SERVER=8.8.8.8

# File containing user specific secrets for accessing the container registry.  Must define:
#
# RBV_USERNAME: GitHub username required for container registry (ghcr.io) login
# RBV_PAT: GitHub Personal Access Token used for both:
#          1. GitHub API authentication (for building/listing/deleting images)
#          2. Container registry authentication (for pulling images)
#          Generate this token at https://github.com/settings/tokens with scopes:
#          - read:packages, write:packages, delete:packages
#          - repo (for workflow dispatch)
export RBRR_GITHUB_PAT_ENV=../secrets/github-ghcr-play.env


# eof
