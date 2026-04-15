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
# Recipe Bottle Windows Handbook - rbhw_docker_wsl_native function

set -euo pipefail

test -z "${ZRBHWDN_SOURCED:-}" || return 0
ZRBHWDN_SOURCED=1

rbhw_docker_wsl_native() {
  zrbhw_sentinel

  buc_doc_brief "Display native Docker daemon installation in WSL (arg: distro-name)"
  buc_doc_shown || return 0

  local z_distro="${BUZ_FOLIO:-}"
  test -n "${z_distro}" || buc_die "rbhw_docker_wsl_native: distro-name required"

  buh_section  "Native Docker Daemon in WSL"
  buyy_cmd_yawp "${z_distro}"; local -r z_distro_yelp="${z_buym_yelp}"
  buh_line "Install and run dockerd natively inside WSL distro ${z_distro_yelp}."
  buh_e
  buh_step1    "Enter the Distribution:"
  buh_code     "wsl -d ${z_distro}"
  buh_e
  buh_step1    "Install Docker Engine:"
  buh_code     "sudo apt update"
  buh_code     "sudo apt install -y docker.io"
  buh_code     "sudo systemctl enable --now docker"
  buh_e
  buh_step1    "Grant Docker Access to Users:"
  buh_line     "Add each user that needs Docker access to the docker group:"
  buh_code     "sudo usermod -aG docker USERNAME"
  buh_line     "Replace USERNAME with each fundus user account."
  buh_e
  buh_section  "Verification:"
  buh_code     "docker ps"
  buh_line     "Expect: empty container list (no errors)."

}

# eof
