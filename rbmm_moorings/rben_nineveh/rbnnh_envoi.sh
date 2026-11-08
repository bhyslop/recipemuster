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
# nineveh envoi
#
# Prints the Kroki URL and a sample request, the port read from the rbrn.env
# beside this script.
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

readonly z_port="${RBRN_ENTRY_PORT_WORKSTATION}"

printf '%s\n' "Kroki-core ready: http://localhost:${z_port}" || exit 1
printf '%s\n' "Core engines: graphviz, plantuml, d2 — POST diagram source to /<engine>/svg" || exit 1
printf '%s\n' "Sample (graphviz):" || exit 1
printf '%s\n' "  echo 'digraph { a -> b }' | curl -s --data-binary @- http://localhost:${z_port}/graphviz/svg" || exit 1
