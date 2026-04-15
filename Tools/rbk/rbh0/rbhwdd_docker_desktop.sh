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
# Recipe Bottle Windows Handbook - rbhw_docker_desktop function

set -euo pipefail

test -z "${ZRBHWDD_SOURCED:-}" || return 0
ZRBHWDD_SOURCED=1

rbhw_docker_desktop() {
  zrbhw_sentinel

  buc_doc_brief "Display Docker Desktop installation procedure"
  buc_doc_shown || return 0

  buh_section  "Docker Desktop Installation"
  buh_line     "Provide Windows-hosted Docker daemon for Windows and Cygwin environments."
  buh_e
  buh_step1    "Download and Install Docker Desktop:"
  buh_link     "" "Docker Desktop for Windows" "https://www.docker.com/products/docker-desktop/"
  buh_e
  buh_step1    "Enable WSL Integration:"
  buyy_ui_yawp "WSL Integration"; local -r z_wsl_int="${z_buym_yelp}"
  buh_line "In Docker Desktop Settings > Resources > ${z_wsl_int}"
  buyy_ui_yawp "Enable integration with my default WSL distro"; local -r z_wsl_distro="${z_buym_yelp}"
  buh_line "Enable ${z_wsl_distro}"
  buh_e
  buh_step1    "Start Docker Desktop:"
  buh_line     "Launch Docker Desktop from the Start menu or system tray."
  buh_e
  buh_section  "Verification:"
  buh_line     "In a Windows PowerShell:"
  buh_code     "docker ps"
  buh_line     "Expect: empty container list (no errors)."

}

# eof
