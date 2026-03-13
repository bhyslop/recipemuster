#!/bin/bash
#
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
# RBRR CLI - Command line interface for RBRR repo regime operations

set -euo pipefail

source "${BURD_BUK_DIR}/buc_command.sh"

######################################################################
# Internal Functions

# Module-scoped arrays and timestamps for BCG file rewrite — populated by refresh commands
ZRBRR_IMAGE_LINES=()
ZRBRR_BINARY_LINES=()
ZRBRR_IMAGE_PINS_REFRESHED_AT=""
ZRBRR_BINARY_PINS_REFRESHED_AT=""

# BCG compliant: write complete RBRG file from image + binary pin arrays, replace atomically
# Caller must populate ZRBRR_IMAGE_LINES, ZRBRR_BINARY_LINES, and both REFRESHED_AT values.
zrbrr_write_rbrg() {
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
    for z_line in "${ZRBRR_IMAGE_LINES[@]}"; do
      echo "${z_line}"
    done
    echo ''
    for z_line in "${ZRBRR_BINARY_LINES[@]}"; do
      echo "${z_line}"
    done
    echo ''
    echo "RBRG_IMAGE_PINS_REFRESHED_AT=${ZRBRR_IMAGE_PINS_REFRESHED_AT}"
    echo "RBRG_BINARY_PINS_REFRESHED_AT=${ZRBRR_BINARY_PINS_REFRESHED_AT}"
    echo ''
    echo '# eof'
  } > "${z_temp_file}" || buc_die "Failed to write temporary RBRG file"
  mv "${z_temp_file}" "${z_rbrg_file}" || buc_die "Failed to replace RBRG file"
}

######################################################################
# Command Functions

# Command: reset - transform RBRR to blank template via line-by-line rewrite
# Pre-fills infrastructure defaults, blanks site-specific fields.
# Uses differential furnish (light path, no regime kindle/enforce).
rbrr_reset() {
  buc_doc_brief "Reset RBRR repo regime to blank template for release qualification"
  buc_doc_shown || return 0

  local -r z_rbrr="${RBBC_rbrr_file}"
  test -f "${z_rbrr}" || buc_die "RBRR file not found: ${z_rbrr}"

  # Discover secrets dir for pre-confirmation inventory
  local z_secrets_dir=""
  z_secrets_dir=$(grep -m1 '^RBRR_SECRETS_DIR=' "${z_rbrr}") || z_secrets_dir=""
  z_secrets_dir="${z_secrets_dir#RBRR_SECRETS_DIR=}"

  bug_section "Marshal Reset"
  bug_t "  Target: ${z_rbrr}"
  bug_e
  bug_t "  RBRR fields blanked (reset to onboarding start):"
  bug_t "    RBRR_DEPOT_PROJECT_ID, RBRR_GAR_REPOSITORY,"
  bug_t "    RBRR_CBV2_CONNECTION_NAME, RBRR_GCB_WORKER_POOL, RBRR_RUBRIC_REPO_URL"
  bug_e
  bug_t "  RBRR fields pre-filled to defaults:"
  bug_t "    RBRR_DNS_SERVER, RBRR_GCB_MACHINE_TYPE, RBRR_GCB_TIMEOUT,"
  bug_t "    RBRR_GCB_MIN_CONCURRENT_BUILDS, RBRR_GCP_REGION,"
  bug_t "    RBRR_VESSEL_DIR, RBRR_SECRETS_DIR"
  bug_e
  bug_t "  Depot credentials DELETED (tied to prior depot):"
  if test -n "${z_secrets_dir}"; then
    local z_preview=""
    local z_any_cred=0
    for z_preview in rbra-governor.env rbra-director.env rbra-retriever.env; do
      if test -f "${z_secrets_dir}/${z_preview}"; then
        bug_t "    ${z_secrets_dir}/${z_preview}"
        z_any_cred=1
      fi
    done
    test "${z_any_cred}" = "1" || bug_t "    (none present)"
  else
    bug_t "    (secrets dir not configured)"
  fi
  bug_e
  bug_t "  Vessel consecrations BLANKED (stale after depot change):"
  local z_np_preview=""
  local z_any_np=0
  for z_np_preview in "${RBBC_dot_dir}"/rbrn_*.env; do
    test -f "${z_np_preview}" || continue
    bug_t "    ${z_np_preview}"
    z_any_np=1
  done
  test "${z_any_np}" = "1" || bug_t "    (no nameplates found)"
  bug_e
  bug_t "  Preserved (payor-scoped, survives depot change):"
  bug_t "    ${z_secrets_dir}/rbro-payor.env"
  bug_e
  buc_require "Proceed with marshal reset?" "reset"

  local -r z_tmp="${z_rbrr}.tmp"
  while IFS= read -r z_line; do
    case "${z_line}" in
      # Pre-selected defaults
      RBRR_DNS_SERVER=*)                    printf '%s\n' "RBRR_DNS_SERVER=8.8.8.8"                     ;;
      RBRR_GCB_MACHINE_TYPE=*)              printf '%s\n' "RBRR_GCB_MACHINE_TYPE=e2-standard-2"         ;;
      RBRR_GCB_TIMEOUT=*)                   printf '%s\n' "RBRR_GCB_TIMEOUT=2700s"                      ;;
      RBRR_GCB_MIN_CONCURRENT_BUILDS=*)     printf '%s\n' "RBRR_GCB_MIN_CONCURRENT_BUILDS=1"            ;;
      RBRR_GCP_REGION=*)                    printf '%s\n' "RBRR_GCP_REGION=us-central1"                 ;;
      RBRR_VESSEL_DIR=*)                    printf '%s\n' "RBRR_VESSEL_DIR=rbev-vessels"                ;;
      RBRR_SECRETS_DIR=*)                   printf '%s\n' "RBRR_SECRETS_DIR=../station-files/secrets"   ;;
      # Site-specific fields blanked
      RBRR_DEPOT_PROJECT_ID=*)              printf '%s\n' "RBRR_DEPOT_PROJECT_ID="                      ;;
      RBRR_GAR_REPOSITORY=*)                printf '%s\n' "RBRR_GAR_REPOSITORY="                        ;;
      RBRR_CBV2_CONNECTION_NAME=*)          printf '%s\n' "RBRR_CBV2_CONNECTION_NAME="                  ;;
      RBRR_GCB_WORKER_POOL=*)               printf '%s\n' "RBRR_GCB_WORKER_POOL="                       ;;
      RBRR_RUBRIC_REPO_URL=*)               printf '%s\n' "RBRR_RUBRIC_REPO_URL="                       ;;
      # Everything else passes through (comments, shebang, blanks)
      *)                                    printf '%s\n' "${z_line}"                                   ;;
    esac
  done < "${z_rbrr}" > "${z_tmp}" && mv "${z_tmp}" "${z_rbrr}"

  # Remove depot-scoped RBRA files (governor, director, retriever).
  # z_secrets_dir already extracted above for pre-confirmation inventory.
  if test -n "${z_secrets_dir}"; then
    local z_rbra=""
    for z_rbra in rbra-governor.env rbra-director.env rbra-retriever.env; do
      if test -f "${z_secrets_dir}/${z_rbra}"; then
        rm "${z_secrets_dir}/${z_rbra}"
        bug_t "  Removed stale depot credential: ${z_rbra}"
      fi
    done
  fi

  # Blank consecration values in all vessel nameplates.
  # Consecrations reference images built against the prior depot — they
  # become stale after reset.  Blanking them causes the onboarding guide
  # to require conjure & vouch before declaring setup complete.
  local z_np=""
  local z_np_tmp=""
  for z_np in "${RBBC_dot_dir}"/rbrn_*.env; do
    test -f "${z_np}" || continue
    z_np_tmp="${z_np}.tmp"
    while IFS= read -r z_line; do
      case "${z_line}" in
        RBRN_SENTRY_CONSECRATION=*)  printf '%s\n' "RBRN_SENTRY_CONSECRATION=" ;;
        RBRN_BOTTLE_CONSECRATION=*)  printf '%s\n' "RBRN_BOTTLE_CONSECRATION=" ;;
        *)                           printf '%s\n' "${z_line}"                  ;;
      esac
    done < "${z_np}" > "${z_np_tmp}" && mv "${z_np_tmp}" "${z_np}"
    bug_t "  Blanked consecrations: ${z_np}"
  done

  bug_t "  Reset complete: ${z_rbrr}"
  bug_e
  bug_t "  Next: verify onboarding guide detects blank state:"
  buc_tabtarget "${RBZ_PAYOR_ONBOARDING}"
  buc_success "Regime reset to blank template"
}

# Command: validate - enrollment-based validation report
rbrr_validate() {
  buc_doc_brief "Validate RBRR repo regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRR repo regime file: ${RBBC_rbrr_file}"
  buv_report RBRR "Repository Regime"
  buc_step "RBRR repo regime valid"
}

# Command: render - diagnostic display of all RBRR fields
rbrr_render() {
  buc_doc_brief "Display diagnostic view of RBRR repo regime configuration"
  buc_doc_shown || return 0

  buv_render RBRR "RBRR - Recipe Bottle Regime Repo"
}

# Command: validate_pins - enrollment-based validation report for RBRG pins
rbrr_validate_pins() {
  buc_doc_brief "Validate RBRG pin regime configuration via enrollment report"
  buc_doc_shown || return 0

  buc_step "Validating RBRG pin file: ${RBBC_rbrg_file}"
  buv_report RBRG "GCB Pins Regime"
  buc_step "RBRG pin regime valid"
}

# Command: render_pins - diagnostic display of all RBRG fields
rbrr_render_pins() {
  buc_doc_brief "Display diagnostic view of RBRG pin regime configuration"
  buc_doc_shown || return 0

  buv_render RBRG "RBRG - GCB Pins Regime"
}

# Command: refresh_gcb_pins - resolve image tags to digests and write complete RBRG file
# BCG compliant: discovers all values, writes complete file from scratch, replaces atomically.
# Every step must succeed or the function dies — no partial updates.
# Requires: docker, jq, curl, BURD_NOW_STAMP set, BURD_TEMP_DIR set
rbrr_refresh_gcb_pins() {
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

    buc_log_args "Extracting ${z_pin_os}/${z_pin_arch} digest from manifest"
    jq -r --arg os "${z_pin_os}" --arg arch "${z_pin_arch}" '
      if type == "array" then
        [.[] | select(.Descriptor.platform.architecture == $arch
                  and .Descriptor.platform.os == $os)][0].Descriptor.digest
      else
        .Descriptor.digest
      end' \
      "${z_manifest_file}" > "${z_digest_file}" \
      || buc_die "Failed to extract digest for ${z_image}:${z_tag}"
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

  # Populate shared state: freshly resolved image pins, pass-through binary pins and its timestamp
  ZRBRR_IMAGE_LINES=("${z_resolved_lines[@]}")
  ZRBRR_BINARY_LINES=(
    "RBRG_SLSA_VERIFIER_URL=\"${RBRG_SLSA_VERIFIER_URL}\""
    "RBRG_SLSA_VERIFIER_SHA256=\"${RBRG_SLSA_VERIFIER_SHA256}\""
  )
  ZRBRR_IMAGE_PINS_REFRESHED_AT="${BURD_NOW_EPOCH}"
  ZRBRR_BINARY_PINS_REFRESHED_AT="${RBRG_BINARY_PINS_REFRESHED_AT}"

  buc_step "Writing complete RBRG pin file (BCG rewrite pattern)"
  local -r z_temp_rbrg="${z_prefix}rbrg_complete.env"
  zrbrr_write_rbrg "${z_vintage}" "${z_temp_rbrg}"

  buc_step "Refresh complete: ${z_updated} updated, ${z_unchanged} unchanged"
}

# Command: refresh_binary_pins - discover latest slsa-verifier release and verify checksum
# Downloads binary, computes local SHA256, fetches SLSA provenance attestation SHA256,
# dies if they disagree (belt and suspenders).
# Requires: curl, jq, shasum, BURD_NOW_STAMP set, BURD_TEMP_DIR set
rbrr_refresh_binary_pins() {
  buc_doc_brief "Discover latest slsa-verifier release, verify checksum, write RBRG pin file"
  buc_doc_shown || return 0

  local -r z_rbrg_file="${RBBC_rbrg_file}"
  test -f "${z_rbrg_file}" || buc_die "RBRG config not found: ${z_rbrg_file}"
  zburd_sentinel

  local -r z_vintage="${BURD_NOW_STAMP}"
  local -r z_prefix="${BURD_TEMP_DIR}/rbrg_binary_refresh_"
  local -r z_releases_file="${z_prefix}releases.json"
  local -r z_releases_stderr="${z_prefix}releases_stderr.txt"
  local -r z_binary_file="${z_prefix}slsa-verifier-linux-amd64"
  local -r z_binary_stderr="${z_prefix}binary_stderr.txt"
  local -r z_attestation_file="${z_prefix}attestation.intoto.jsonl"
  local -r z_attestation_stderr="${z_prefix}attestation_stderr.txt"
  local -r z_local_sha_file="${z_prefix}local_sha256.txt"
  local -r z_provenance_sha_file="${z_prefix}provenance_sha256.txt"

  # Discover latest release version from GitHub Releases API
  buc_step "Discovering latest slsa-verifier release from GitHub"
  curl -sS \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    "https://api.github.com/repos/slsa-framework/slsa-verifier/releases/latest" \
    -o "${z_releases_file}" 2>"${z_releases_stderr}" \
    || buc_die "Failed to fetch latest release — see ${z_releases_stderr}"

  local z_tag=""
  z_tag=$(jq -r '.tag_name // empty' "${z_releases_file}") \
    || buc_die "Failed to extract tag_name from release response"
  test -n "${z_tag}" || buc_die "GitHub returned empty tag_name for latest release"
  buc_info "slsa-verifier latest release: ${z_tag}"

  local -r z_binary_url="https://github.com/slsa-framework/slsa-verifier/releases/download/${z_tag}/slsa-verifier-linux-amd64"
  local -r z_attestation_url="${z_binary_url}.intoto.jsonl"

  # Download the binary
  buc_step "Downloading slsa-verifier binary (${z_tag})"
  curl -sS -L \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time 300 \
    "${z_binary_url}" \
    -o "${z_binary_file}" 2>"${z_binary_stderr}" \
    || buc_die "Failed to download slsa-verifier binary — see ${z_binary_stderr}"

  # Compute local SHA256
  buc_step "Computing local SHA256 checksum"
  shasum -a 256 "${z_binary_file}" | cut -d' ' -f1 > "${z_local_sha_file}" \
    || buc_die "Failed to compute SHA256 of downloaded binary"
  local z_local_sha=""
  z_local_sha=$(<"${z_local_sha_file}")
  test -n "${z_local_sha}" || buc_die "Empty local SHA256"

  # Download the SLSA provenance attestation
  buc_step "Downloading SLSA provenance attestation"
  curl -sS -L \
    --connect-timeout "${RBCC_CURL_CONNECT_TIMEOUT_SEC}" --max-time "${RBCC_CURL_MAX_TIME_SEC}" \
    "${z_attestation_url}" \
    -o "${z_attestation_file}" 2>"${z_attestation_stderr}" \
    || buc_die "Failed to download attestation — see ${z_attestation_stderr}"

  # Extract SHA256 from in-toto provenance subject
  buc_step "Extracting SHA256 from provenance attestation"
  jq -r '.payload | @base64d | fromjson | [.subject[] | select(.name == "slsa-verifier-linux-amd64")][0].digest.sha256' \
    "${z_attestation_file}" > "${z_provenance_sha_file}" \
    || buc_die "Failed to extract SHA256 from attestation"
  local z_provenance_sha=""
  z_provenance_sha=$(<"${z_provenance_sha_file}")
  test -n "${z_provenance_sha}" || buc_die "Empty provenance SHA256"

  # Belt and suspenders: local checksum must match provenance
  buc_step "Verifying local checksum matches provenance attestation"
  buc_info "Local SHA256:      ${z_local_sha}"
  buc_info "Provenance SHA256: ${z_provenance_sha}"
  test "${z_local_sha}" = "${z_provenance_sha}" \
    || buc_die "CHECKSUM MISMATCH: local=${z_local_sha} provenance=${z_provenance_sha}"
  buc_info "Checksums match"

  # Log changes vs current values
  local z_updated=0
  local z_unchanged=0
  if test "${RBRG_SLSA_VERIFIER_URL}" = "${z_binary_url}" \
       -a "${RBRG_SLSA_VERIFIER_SHA256}" = "${z_local_sha}"; then
    buc_info "RBRG_SLSA_VERIFIER: unchanged (${z_tag})"
    z_unchanged=1
  else
    buc_info "RBRG_SLSA_VERIFIER_URL: ${RBRG_SLSA_VERIFIER_URL} -> ${z_binary_url}"
    buc_info "RBRG_SLSA_VERIFIER_SHA256: ${RBRG_SLSA_VERIFIER_SHA256} -> ${z_local_sha}"
    z_updated=1
  fi

  # Populate shared state: pass-through image pins and its timestamp, freshly resolved binary pins
  ZRBRR_IMAGE_LINES=(
    "RBRG_ORAS_IMAGE_REF=\"${RBRG_ORAS_IMAGE_REF}\""
    "RBRG_GCLOUD_IMAGE_REF=\"${RBRG_GCLOUD_IMAGE_REF}\""
    "RBRG_DOCKER_IMAGE_REF=\"${RBRG_DOCKER_IMAGE_REF}\""
    "RBRG_ALPINE_IMAGE_REF=\"${RBRG_ALPINE_IMAGE_REF}\""
    "RBRG_SYFT_IMAGE_REF=\"${RBRG_SYFT_IMAGE_REF}\""
    "RBRG_BINFMT_IMAGE_REF=\"${RBRG_BINFMT_IMAGE_REF}\""
    "RBRG_SKOPEO_IMAGE_REF=\"${RBRG_SKOPEO_IMAGE_REF}\""
  )
  ZRBRR_BINARY_LINES=(
    "RBRG_SLSA_VERIFIER_URL=\"${z_binary_url}\""
    "RBRG_SLSA_VERIFIER_SHA256=\"${z_local_sha}\""
  )
  ZRBRR_IMAGE_PINS_REFRESHED_AT="${RBRG_IMAGE_PINS_REFRESHED_AT}"
  ZRBRR_BINARY_PINS_REFRESHED_AT="${BURD_NOW_EPOCH}"

  buc_step "Writing complete RBRG pin file (BCG rewrite pattern)"
  local -r z_temp_rbrg="${z_prefix}rbrg_complete.env"
  zrbrr_write_rbrg "${z_vintage}" "${z_temp_rbrg}"

  buc_step "Refresh complete: ${z_updated} updated, ${z_unchanged} unchanged"
}

######################################################################
# Furnish and Main

zrbrr_furnish() {
  buc_doc_env "BURD_BUK_DIR          " "BUK module directory (dispatch-provided)"
  buc_doc_env "BURD_TOOLS_DIR        " "Project tools root directory (dispatch-provided)"
  buc_doc_env_done || return 0

  local z_command="${1:-}"

  # Light sources (always)
  source "${BURD_CONFIG_DIR}/rbbc_constants.sh"
  local z_rbk_kit_dir="${BURD_TOOLS_DIR}/${RBBC_kit_subdir}"

  # Differential furnish: reset needs guide + zipper, everything else needs regime
  case "${z_command}" in
    rbrr_reset)
      source "${BURD_BUK_DIR}/bug_guide.sh"      || buc_die "Failed to source bug_guide.sh"
      source "${BURD_BUK_DIR}/buz_zipper.sh"     || buc_die "Failed to source buz_zipper.sh"
      source "${z_rbk_kit_dir}/rbz_zipper.sh"    || buc_die "Failed to source rbz_zipper.sh"
      zbuz_kindle
      zrbz_kindle
      ;;
    *)
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
      zrbrr_enforce
      zrbrg_kindle
      zrbrg_enforce
      zrbdc_kindle

      zbupr_kindle
      ;;
  esac
}

buc_execute rbrr_ "Recipe Bottle Repository Regime" zrbrr_furnish "$@"

# eof
