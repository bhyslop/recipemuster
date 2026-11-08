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
# srjcl envoi
#
# Prints the JupyterLab URL, its port read from the rbrn.env beside this
# script, then warns if the operator's shell did not export ANTHROPIC_API_KEY
# (in which case notebook cells that call the Anthropic API will fail
# authentication).
#
# Self-contained by contract: it sources no kit module, and every nameplate
# value it prints is read from the rbrn.env beside it, never from the
# environment it runs in.

set -euo pipefail

readonly z_self="${BASH_SOURCE[0]}"
z_dir="."
case "${z_self}" in
  */*) z_dir="${z_self%/*}" ;;
esac
readonly z_dir

source "${z_dir}/rbrn.env"

readonly z_yellow=$'\033[33m'
readonly z_bold=$'\033[1m'
readonly z_reset=$'\033[0m'

printf 'JupyterLab ready: http://localhost:%s/lab\n' "${RBRN_ENTRY_PORT_WORKSTATION}" || exit 1

if test -z "${ANTHROPIC_API_KEY:-}"; then
  printf '%sWARNING:%s ANTHROPIC_API_KEY is not set in your shell — Anthropic API calls in the notebook will fail.\n' "${z_yellow}" "${z_reset}" || exit 1
  printf '  Remediation: %sexport ANTHROPIC_API_KEY=sk-...%s in your shell, then restart srjcl to pick it up.\n' "${z_bold}" "${z_reset}" || exit 1
fi
