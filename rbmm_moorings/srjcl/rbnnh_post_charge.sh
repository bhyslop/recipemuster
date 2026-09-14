#!/usr/bin/env bash
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
#
# srjcl post-charge hook
#
# Warns if the operator's shell did not export ANTHROPIC_API_KEY (in which case
# notebook cells that call the Anthropic API will fail authentication). The
# JupyterLab URL is surfaced declaratively via rbnnh_charge_note.txt — this hook
# retains only the conditional check, which a note cannot express.
#
# Self-contained by contract: no BUK/RBK utility-module dependencies. Inherits
# RBRN_*/RBRR_* environment from the charge process.

set -eu

readonly z_yellow=$'\033[33m'
readonly z_bold=$'\033[1m'
readonly z_reset=$'\033[0m'

if test -z "${ANTHROPIC_API_KEY:-}"; then
  printf '%sWARNING:%s ANTHROPIC_API_KEY is not set in your shell — Anthropic API calls in the notebook will fail.\n' "${z_yellow}" "${z_reset}"
  printf '  Remediation: %sexport ANTHROPIC_API_KEY=sk-...%s in your shell, then re-charge srjcl to pick it up.\n' "${z_bold}" "${z_reset}"
fi
