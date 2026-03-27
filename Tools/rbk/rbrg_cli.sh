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
# RBRG CLI - Command line interface for RBRG GCB pin regime operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Internal Functions

# Module-scoped arrays and timestamp for BCG file rewrite — populated by refresh commands
ZRBRG_IMAGE_LINES=()
ZRBRG_IMAGE_PINS_REFRESHED_AT=""

# BCG compliant: write complete RBRG file from image pin array, replace atomically
# Caller must populate ZRBRG_IMAGE_LINES and ZRBRG_IMAGE_PINS_REFRESHED_AT.
zrbrg_write_rbrg() {
  local z_vintage="$1"
  local z_temp_file="$2"
  local -r z_rbrg_file="${RBBC_rbrg_file}"
  {
    echo '#!/bin/bash'
    echo '# Copyright 2026 Scale Invariant, Inc.'
    echo '#'
    echo '# Licensed under the Apache License, Version 2.0 (the "License");'
    echo '# you may not use this file except in compliance with the License.'
    echo '# You may obtain a copy of the License at'
    echo '#'
    echo '#     http://www.apache.org/licenses/LICENSE-2.0'
    echo '#'
    echo '# Unless required by applicable law or agreed to in writing, software'
    echo '# distributed under the License is distributed on an "AS IS" BASIS,'
    echo '# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.'
    echo '# See the License for the specific language governing permissions and'
    echo '# limitations under the License.'
    echo '#'
    echo '# Author: Brad Hyslop <bhyslop@scaleinvariant.org>'
    echo '#'
    echo "# RBRG - GCB Pins (refreshed ~${z_vintage})"
    echo ''
    local z_line=""
    for z_line in "${ZRBRG_IMAGE_LINES[@]}"; do
      echo "${z_line}"
    done
    echo ''
    echo "RBRG_IMAGE_PINS_REFRESHED_AT=${ZRBRG_IMAGE_PINS_REFRESHED_AT}"
    echo ''
    echo '# eof'
  } > "${z_temp_file}" || buc_die "Failed to write temporary RBRG file"
  mv "${z_temp_file}" "${z_rbrg_file}" || buc_die "Failed to replace RBRG file"
}

######################################################################
# Command Functions

# Command: validate_pins - enrollment-based validation report for RBRG pins
rbrg_validate_pins() {
  buc_doc_brief "Validate RBRG pin regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRG pin file: ${RBBC_rbrg_file}"
  buv_report RBRG "GCB Pins Regime"
  buc_step "RBRG pin regime valid"
}

# Command: render_pins - diagnostic display of all RBRG fields
rbrg_render_pins() {
  buc_doc_brief "Display diagnostic view of RBRG pin regime configuration"
  buc_doc_shown || return 0

  buv_render RBRG "RBRG - GCB Pins Regime"
}

# Command: refresh_gcb_pins - resolve image tags to digests and write complete RBRG file
# BCG compliant: discovers all values, writes complete file from scratch, replaces atomically.
# Every step must succeed or the function dies — no partial updates.
# Requires: docker, jq, curl, BURD_NOW_STAMP set, BURD_TEMP_DIR set
rbrg_refresh_gcb_pins() {
  buc_doc_brief "Resolve image tags to digests and write complete RBRG pin file"
  buc_doc_shown || return 0

  local -r z_rbrg_file="${RBBC_rbrg_file}"
  test -f "${z_rbrg_file}" || buc_die "RBRG config not found: ${z_rbrg_file}"
  zburd_sentinel

  buc_countdown 5 "NOTE: Docker anonymous login is heavily rate limited.  Try again in 6 hours if you see that fail.  Continuing in:"

  local -r z_vintage="${BURD_NOW_STAMP}"
  local -r z_prefix="${BURD_TEMP_DIR}/rbrg_refresh_"

  local z_oras_token_file="${z_prefix}oras_token.json"
  local z_oras_tags_file="${z_prefix}oras_tags.json"
  local z_oras_token_value_file="${z_prefix}oras_token_value.txt"
  local z_oras_tag_file="${z_prefix}oras_tag.txt"
  local z_oras_token_stderr="${z_prefix}oras_token_stderr.txt"
  local z_oras_tags_stderr="${z_prefix}oras_tags_stderr.txt"

  # Discover latest oras stable version from GHCR API.
  # oras doesn't publish a :latest tag, so we must find the newest semver release.
  # GHCR requires a bearer token even for public images.
  buc_step "Discovering latest oras version from GHCR (no :latest tag published)"

  buc_log_args "Obtaining GHCR bearer token (anonymous, scoped to oras repo)"
  curl -sS \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    "https://ghcr.io/token?scope=repository:oras-project/oras:pull&service=ghcr.io" \
    -o "${z_oras_token_file}" 2>"${z_oras_token_stderr}" \
    || buc_die "Failed to fetch GHCR bearer token — see ${z_oras_token_stderr}"

  jq -r '.token // empty' "${z_oras_token_file}" > "${z_oras_token_value_file}" \
    || buc_die "Failed to extract token from GHCR response"
  local z_oras_token
  z_oras_token=$(<"${z_oras_token_value_file}")
  test -n "${z_oras_token}" || buc_die "GHCR returned empty bearer token"

  buc_log_args "Fetching tag list using bearer token"
  curl -sS \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    -H "Authorization: Bearer ${z_oras_token}" \
    "https://ghcr.io/v2/oras-project/oras/tags/list?n=1000" \
    -o "${z_oras_tags_file}" 2>"${z_oras_tags_stderr}" \
    || buc_die "Failed to fetch oras tags from GHCR — see ${z_oras_tags_stderr}"

  buc_log_args "Extracting newest stable semver tag"
  # Filters to exact vN.N.N (no pre-release suffixes), sorts numerically, takes last
  jq -r '
    [.tags[] | select(test("^v[0-9]+\\.[0-9]+\\.[0-9]+$"))]
    | map(ltrimstr("v") | split(".") | map(tonumber))
    | sort_by(.[0], .[1], .[2])
    | last
    | "v\(.[0]).\(.[1]).\(.[2])"
  ' "${z_oras_tags_file}" > "${z_oras_tag_file}" \
    || buc_die "Failed to extract semver tag from oras tags response"
  local z_oras_tag
  z_oras_tag=$(<"${z_oras_tag_file}")
  test -n "${z_oras_tag}" -a "${z_oras_tag}" != "null" \
    || buc_die "No stable oras semver tag found in GHCR tag list"
  buc_info "oras discovered tag: ${z_oras_tag}"

  # Image specifications: VARNAME|BASE_IMAGE|TAG
  # Most images use :latest which always points to newest version.
  # oras uses discovered semver tag above (no :latest published).
  local z_specs=(
    "RBRG_ORAS_IMAGE_REF|ghcr.io/oras-project/oras|${z_oras_tag}"
    "RBRG_GCLOUD_IMAGE_REF|gcr.io/cloud-builders/gcloud|latest"
    "RBRG_DOCKER_IMAGE_REF|gcr.io/cloud-builders/docker|latest"
    "RBRG_ALPINE_IMAGE_REF|docker.io/library/alpine|latest"
    "RBRG_SYFT_IMAGE_REF|docker.io/anchore/syft|latest"
    "RBRG_BINFMT_IMAGE_REF|docker.io/tonistiigi/binfmt|latest"
    "RBRG_SKOPEO_IMAGE_REF|quay.io/skopeo/stable|latest"
  )

  buc_step "Refreshing GCB tool image pins (vintage: ~${z_vintage})"

  # Resolve all digests, collecting results for complete file write
  local -a z_resolved_lines=()
  local z_updated=0
  local z_unchanged=0

  local z_spec=""
  local z_varname=""
  local z_image=""
  local z_tag=""
  local z_manifest_file=""
  local z_digest_file=""
  local z_digest=""
  local z_full_ref=""
  # GCB runs on linux/amd64 — pin digests must match this platform
  local z_pin_os="linux"
  local z_pin_arch="amd64"
  local z_inspect_stderr=""
  local z_index=0
  for z_spec in "${z_specs[@]}"; do
    IFS='|' read -r z_varname z_image z_tag <<< "${z_spec}"

    z_manifest_file="${z_prefix}${z_index}_manifest.json"
    z_digest_file="${z_prefix}${z_index}_digest.txt"
    z_inspect_stderr="${z_prefix}${z_index}_inspect_stderr.txt"

    buc_step "Inspecting ${z_image}:${z_tag}"

    docker manifest inspect --verbose "${z_image}:${z_tag}" > "${z_manifest_file}" 2>"${z_inspect_stderr}" \
      || buc_die "Failed to fetch manifest for ${z_image}:${z_tag} — see ${z_inspect_stderr}"

    buc_log_args "Extracting ${z_pin_os}/${z_pin_arch} digest (normalize-then-filter)"
    jq -r --arg os "${z_pin_os}" --arg arch "${z_pin_arch}" '
      first([.] | flatten | .[].Descriptor
        | select(.platform.architecture == $arch and .platform.os == $os)
        | .digest)' \
      "${z_manifest_file}" > "${z_digest_file}" \
      || buc_die "Failed to extract ${z_pin_os}/${z_pin_arch} digest for ${z_image}:${z_tag} (single-platform or multi-arch)"
    z_digest=$(<"${z_digest_file}")
    test -n "${z_digest}" || buc_die "Empty digest for ${z_image}:${z_tag}"
    z_full_ref="${z_image}@${z_digest}"

    # Log changes vs current values
    local z_old_ref="${!z_varname:-}"
    if test "${z_old_ref}" = "${z_full_ref}"; then
      buc_info "${z_varname}: unchanged"
      z_unchanged=$((z_unchanged + 1))
    else
      buc_info "${z_varname}: ${z_old_ref} -> ${z_full_ref}"
      z_updated=$((z_updated + 1))
    fi

    z_resolved_lines+=("${z_varname}=\"${z_full_ref}\"")
    z_index=$((z_index + 1))
  done

  # Populate shared state: freshly resolved image pins
  ZRBRG_IMAGE_LINES=("${z_resolved_lines[@]}")
  ZRBRG_IMAGE_PINS_REFRESHED_AT="${BURD_NOW_EPOCH}"

  buc_step "Writing complete RBRG pin file (BCG rewrite pattern)"
  local -r z_temp_rbrg="${z_prefix}rbrg_complete.env"
  zrbrg_write_rbrg "${z_vintage}" "${z_temp_rbrg}"

  buc_step "Refresh complete: ${z_updated} updated, ${z_unchanged} unchanged"
}

######################################################################
# Furnish and Main

zrbrg_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  source "${BURD_BUK_DIR}/buv_validation.sh"
  source "${BURD_BUK_DIR}/burd_regime.sh"
  source "${z_rbk_kit_dir}/rbcc_Constants.sh"
  source "${z_rbk_kit_dir}/rbrr_regime.sh"
  source "${z_rbk_kit_dir}/rbrg_regime.sh"
  source "${z_rbk_kit_dir}/rbdc_DerivedConstants.sh"
  source "${RBBC_rbrr_file}"
  source "${RBBC_rbrg_file}"
  source "${BURD_BUK_DIR}/bupr_PresentationRegime.sh"

  zbuv_kindle
  zburd_kindle
  zburd_enforce
  zrbcc_kindle

  zrbrr_kindle
  zrbrg_kindle
  zrbrg_enforce
  zrbdc_kindle

  zbupr_kindle
}

buc_execute rbrg_ "Recipe Bottle GCB Pins Regime" zrbrg_furnish "$@"

# eof
