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
# Recipe Bottle Depot Regime Bespoke - Tripwire helpers
#
# Implements the depot-immutable tripwire: at end of successful levy
# the Payor inscribes a FROM-scratch image carrying .rbk/rbrd.env to
# GAR at rbi_df/rbrd:tripwire. Every cloud-submitting command pulls
# that image and exact-byte-diffs the file before submitting, so any
# post-levy hand-edit of rbrd.env dies loudly instead of silently
# differing from the depot's worker-pool quotas, region, and identity.
#
# Public functions:
#   rbrd_inscribe <bearer_token>   — host-side push at end of levy
#   rbrd_check    <bearer_token>   — pre-submit existence + byte-diff
#
# Module convention: rbn{R}{X}_ where R=d (depot regime) and X=b
# (bespoke implementation). See CLAUDE.md "Prefix Naming Discipline".

set -euo pipefail

# Multiple inclusion detection
test -z "${ZRBNDB_SOURCED:-}" || buc_die "Module rbndb multiply sourced - check sourcing hierarchy"
ZRBNDB_SOURCED=1

######################################################################
# Internal Functions (zrbndb_*)

zrbndb_kindle() {
  test -z "${ZRBNDB_KINDLED:-}" || buc_die "Module rbndb already kindled"

  buc_log_args "Ensure dependencies are kindled first"
  zrbgc_sentinel
  zrbgl_sentinel
  zrbrd_sentinel
  zrbdc_sentinel

  # Tripwire image FQN: composed once at kindle time from the depot's
  # GAR coordinates + the rbi_df depot-facts namespace.
  # Shape: <region>-docker.pkg.dev/<depot-project>/<gar-repo>/rbi_df/rbrd:tripwire
  readonly ZRBNDB_REGISTRY_HOST="${RBRD_GCP_REGION}${RBGC_GAR_HOST_SUFFIX}"
  readonly ZRBNDB_TRIPWIRE_IMAGE="${ZRBNDB_REGISTRY_HOST}/${RBDC_DEPOT_PROJECT_ID}/${RBDC_GAR_REPOSITORY}/${RBGL_DEPOT_FACTS_ROOT}/rbrd:tripwire"

  readonly ZRBNDB_KINDLED=1
}

zrbndb_sentinel() {
  test "${ZRBNDB_KINDLED:-}" = "1" || buc_die "Module rbndb not kindled - call zrbndb_kindle first"
}

# Authenticate the host docker client to the depot's GAR registry
# using a bearer token. Idempotent — repeated calls refresh credential.
# Args: bearer_token
zrbndb_docker_login() {
  local -r z_token="${1}"
  printf '%s' "${z_token}" \
    | docker login -u oauth2accesstoken --password-stdin "${ZRBNDB_REGISTRY_HOST}" >/dev/null 2>&1 \
    || buc_die "Docker login to ${ZRBNDB_REGISTRY_HOST} failed (bearer token rejected)"
}

######################################################################
# Public Functions

# Inscribe the tripwire image at end of successful levy.
# Pre-push existence guard: image present → fatal (depot already
# inscribed; must be unmade and relevied to refresh).
# Args: bearer_token (Payor OAuth access token with GAR write)
rbrd_inscribe() {
  zrbndb_sentinel

  local -r z_token="${1:-}"
  test -n "${z_token}" || buc_die "rbrd_inscribe: bearer token required (arg 1)"

  buc_step "Inscribe RBRD tripwire image to GAR"
  buc_log_args "Target: ${ZRBNDB_TRIPWIRE_IMAGE}"

  zrbndb_docker_login "${z_token}"

  buc_log_args "Pre-push existence guard via docker manifest inspect"
  if docker manifest inspect "${ZRBNDB_TRIPWIRE_IMAGE}" >/dev/null 2>&1; then
    buc_warn "Tripwire image already inscribed: ${ZRBNDB_TRIPWIRE_IMAGE}"
    buc_info "RBRD is depot-time-immutable. To refresh the tripwire, the depot"
    buc_info "must be unmade and relevied:"
    buc_info "  rbw-dU \$(rbw-dl)        # unmake the depot"
    buc_info "  rbw-dL                  # relevy with new rbrd.env"
    buc_die "Cannot inscribe over existing tripwire (would mask drift)"
  fi

  buc_log_args "Build FROM-scratch image carrying .rbk/rbrd.env"
  local -r z_build_dir="${BURD_TEMP_DIR}/rbndb_inscribe"
  rm -rf "${z_build_dir}"
  mkdir -p "${z_build_dir}" || buc_die "Failed to create build dir ${z_build_dir}"

  cp "${RBBC_rbrd_file}" "${z_build_dir}/rbrd.env" \
    || buc_die "Failed to copy ${RBBC_rbrd_file} into build context"

  printf '%s\n' \
    'FROM scratch' \
    'COPY rbrd.env /rbrd.env' \
    > "${z_build_dir}/Dockerfile" \
    || buc_die "Failed to write Dockerfile"

  docker build -t "${ZRBNDB_TRIPWIRE_IMAGE}" "${z_build_dir}" >/dev/null \
    || buc_die "docker build failed for ${ZRBNDB_TRIPWIRE_IMAGE}"

  buc_log_args "Push tripwire image to GAR"
  docker push "${ZRBNDB_TRIPWIRE_IMAGE}" >/dev/null \
    || buc_die "docker push failed for ${ZRBNDB_TRIPWIRE_IMAGE}"

  buc_success "Tripwire inscribed: ${ZRBNDB_TRIPWIRE_IMAGE}"
}

# Check local .rbk/rbrd.env against the inscribed tripwire image
# before submitting cloud work. Exact-byte mismatch, missing image,
# or registry/auth failure all fatal with recovery guidance.
# Args: bearer_token (role-agnostic — any token with GAR read access)
rbrd_check() {
  zrbndb_sentinel

  local -r z_token="${1:-}"
  test -n "${z_token}" || buc_die "rbrd_check: bearer token required (arg 1)"

  buc_log_args "Tripwire check against: ${ZRBNDB_TRIPWIRE_IMAGE}"

  zrbndb_docker_login "${z_token}"

  buc_log_args "Existence check via docker manifest inspect"
  if ! docker manifest inspect "${ZRBNDB_TRIPWIRE_IMAGE}" >/dev/null 2>&1; then
    buc_warn "Tripwire image absent from GAR: ${ZRBNDB_TRIPWIRE_IMAGE}"
    buc_info "Depot has not been inscribed, or the tripwire was jettisoned."
    buc_info "Recovery: re-levy the depot to mint the tripwire."
    buc_info "  rbw-dU \$(rbw-dl)        # unmake (if a stale depot exists)"
    buc_info "  rbw-dL                  # levy + inscribe tripwire"
    buc_die "Cannot proceed without tripwire — depot regime drift cannot be detected"
  fi

  buc_log_args "Pull tripwire image"
  docker pull "${ZRBNDB_TRIPWIRE_IMAGE}" >/dev/null 2>&1 \
    || buc_die "docker pull failed for ${ZRBNDB_TRIPWIRE_IMAGE}"

  local -r z_extract_dir="${BURD_TEMP_DIR}/rbndb_check"
  local -r z_inscribed_file="${z_extract_dir}/rbrd.env.inscribed"
  rm -rf "${z_extract_dir}"
  mkdir -p "${z_extract_dir}" || buc_die "Failed to create extract dir ${z_extract_dir}"

  local z_cid
  z_cid=$(docker create "${ZRBNDB_TRIPWIRE_IMAGE}") \
    || buc_die "docker create failed for ${ZRBNDB_TRIPWIRE_IMAGE}"

  if ! docker cp "${z_cid}:/rbrd.env" "${z_inscribed_file}"; then
    docker rm "${z_cid}" >/dev/null 2>&1 || true
    buc_die "docker cp failed extracting /rbrd.env from tripwire image"
  fi

  docker rm "${z_cid}" >/dev/null 2>&1 || true

  # cmp is authoritative for byte-match; diff is for human display.
  if ! cmp -s "${z_inscribed_file}" "${RBBC_rbrd_file}"; then
    buc_warn "RBRD drift detected"
    buc_info "Local ${RBBC_rbrd_file} differs from the depot-inscribed copy."
    buc_info "Diff (inscribed → local):"
    diff -u "${z_inscribed_file}" "${RBBC_rbrd_file}" >&2 || true
    buc_info "Recovery options:"
    buc_info "  (a) Restore .rbk/rbrd.env to match the inscribed copy (preserves depot)."
    buc_info "  (b) Unmake and re-levy the depot if rbrd.env genuinely needs to change:"
    buc_info "        rbw-dU \$(rbw-dl)"
    buc_info "        rbw-dL"
    buc_die "Refusing to submit cloud work against drifted depot regime"
  fi

  buc_log_args "Tripwire match — depot regime aligned"
}

# eof
