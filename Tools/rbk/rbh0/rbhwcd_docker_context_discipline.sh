#!/bin/bash
#
# Copyright 2026 Scale Invariant, Inc.
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
# Recipe Bottle Windows Handbook - rbhw_docker_context_discipline function

set -euo pipefail

test -z "${ZRBHWCD_SOURCED:-}" || return 0
ZRBHWCD_SOURCED=1

rbhw_docker_context_discipline() {
  zrbhw_sentinel

  buc_doc_brief "Display deterministic Docker daemon selection procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Context Discipline"
  buh_line     "Ensure deterministic daemon selection across environments."
  buh_e
  buh_section  "Preconditions:"
  buh_line     "- Docker Desktop installed (docker-desktop procedure)"
  buh_line     "- Native dockerd in WSL (docker-wsl-native procedure)"
  buh_e
  buh_step1    "Inside WSL — Create Named Context:"
  buyy_cmd_yawp "${ZRBHW_DOCKER_CONTEXT}"; buh_line "Create context ${z_buym_yelp} for the native daemon:"
  buh_code     "docker context create ${ZRBHW_DOCKER_CONTEXT} --docker \"host=unix:///var/run/docker.sock\""
  buh_code     "docker context use ${ZRBHW_DOCKER_CONTEXT}"
  buh_e
  buh_step1    "On Windows (PowerShell) — Confirm Default Context:"
  buh_line     "Ensure the default context is active (Docker Desktop):"
  buh_code     "docker context use default"
  buh_e
  buh_section  "Result:"
  buh_line     "- WSL shells use native daemon (via context)"
  buh_line     "- Windows and Cygwin use Docker Desktop daemon (default)"
  buh_e
  buh_section  "Verification:"
  buh_code     "docker context ls"
  buh_code     "docker info | grep \"Server\""
  buh_line     "Expect: active context matches the environment."

}

# eof
