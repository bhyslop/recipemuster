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
# BUK Windows Commands - Command Line Interface
#
# Furnish: needs buhw tinder (Windows constants) and buc_require.
# No regime stack — commands consume constants directly.

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

zbuwc_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_CONFIG_DIR       " "BUK config directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${BURD_BUK_DIR}/burc_regime.sh"
  source "${BURD_BUK_DIR}/burs_regime.sh"
  source "${BURD_BUK_DIR}/buf_fact.sh"
  source "${BURD_BUK_DIR}/burn_regime.sh"
  source "${BURD_BUK_DIR}/buhw_windows.sh"
  source "${BURD_BUK_DIR}/buwc_windows.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce

  source "${BURD_REGIME_FILE}" || buc_die "Failed to source BURC: ${BURD_REGIME_FILE}"

  zburc_kindle
  zburc_enforce

  source "${BURD_STATION_FILE}" || buc_die "Failed to source BURS: ${BURD_STATION_FILE}"

  zburs_kindle
  zburs_enforce

  zbuhw_kindle
  zbuwc_kindle

  # If BUZ_FOLIO is set, load and kindle the specified profile
  if test -n "${BUZ_FOLIO:-}"; then
    local -r z_profile_file="${BURD_CONFIG_DIR}/users/${BURS_USER}/${BUZ_FOLIO}/burn.env"
    test -f "${z_profile_file}" || buc_die "BURN profile not found: ${z_profile_file}"
    source "${z_profile_file}" || buc_die "Failed to source BURN: ${z_profile_file}"
    zburn_kindle
    zburn_enforce
  fi
}

buc_execute buwc_ "Windows Commands" zbuwc_furnish "$@"

# eof
