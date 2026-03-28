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
# RBTCRV - Regime validation test cases for RBTB testbench
#
# Exercises RBRR (repo), RBRV (vessel), and RBRN (nameplate) validators
# with synthetic bad inputs (negative tests) and real config files
# (positive tests).  All tests are pure local — no GCP, no containers,
# no network.

set -euo pipefail

######################################################################
# RBRR negative test helpers (override inherited RBRR_ var → kindle → enforce)
#
# Baste sources RBBC_rbrr_file without kindling, so RBRR_ vars are set
# but NOT readonly.  Each helper runs in a subshell (via zbuto_invoke),
# overrides one value, then kindles+enforces.

zrbtcrv_rbrr_missing_project_id() {
  unset RBRR_DEPOT_PROJECT_ID
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrr_bad_timeout() {
  export RBRR_GCB_TIMEOUT="1200"
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrr_bad_pool_stem() {
  export RBRR_GCB_POOL_STEM="BAD_POOL_NAME"
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrr_unexpected_var() {
  export RBRR_BOGUS="foo"
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrr_bad_vessel_dir() {
  export RBRR_VESSEL_DIR="/tmp/nonexistent-rbtcrv-vessel-dir"
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrr_bad_secrets_dir() {
  export RBRR_SECRETS_DIR="/tmp/nonexistent-rbtcrv-secrets-dir"
  zrbrr_kindle
  zrbrr_enforce
}

######################################################################
# RBRR negative test cases

rbtcrv_rbrr_missing_project_id_tcase() {
  buto_trace "RBRR: missing RBRR_DEPOT_PROJECT_ID must fail"
  buto_unit_expect_fatal zrbtcrv_rbrr_missing_project_id
}

rbtcrv_rbrr_bad_timeout_tcase() {
  buto_trace "RBRR: GCB_TIMEOUT without 's' suffix must fail"
  buto_unit_expect_fatal zrbtcrv_rbrr_bad_timeout
}

rbtcrv_rbrr_bad_pool_stem_tcase() {
  buto_trace "RBRR: invalid GCB_POOL_STEM format must fail"
  buto_unit_expect_fatal zrbtcrv_rbrr_bad_pool_stem
}

rbtcrv_rbrr_unexpected_var_tcase() {
  buto_trace "RBRR: unexpected RBRR_BOGUS must fail scope sentinel"
  buto_unit_expect_fatal zrbtcrv_rbrr_unexpected_var
}

rbtcrv_rbrr_bad_vessel_dir_tcase() {
  buto_trace "RBRR: non-existent RBRR_VESSEL_DIR must fail"
  buto_unit_expect_fatal zrbtcrv_rbrr_bad_vessel_dir
}

rbtcrv_rbrr_bad_secrets_dir_tcase() {
  buto_trace "RBRR: non-existent RBRR_SECRETS_DIR must fail"
  buto_unit_expect_fatal zrbtcrv_rbrr_bad_secrets_dir
}

######################################################################
# RBRV baseline helpers — valid synthetic configurations

zrbtcrv_rbrv_baseline_conjure() {
  export RBRV_SIGIL="test-vessel"
  export RBRV_DESCRIPTION="Test vessel for validation"
  export RBRV_VESSEL_MODE="conjure"
  export RBRV_CONJURE_DOCKERFILE="path/to/Dockerfile"
  export RBRV_CONJURE_BLDCONTEXT="path/to"
  export RBRV_CONJURE_PLATFORMS="linux/amd64"
}

zrbtcrv_rbrv_baseline_bind() {
  export RBRV_SIGIL="test-vessel"
  export RBRV_DESCRIPTION="Test vessel for validation"
  export RBRV_VESSEL_MODE="bind"
  export RBRV_BIND_IMAGE="us-docker.pkg.dev/project/repo/image:latest"
}

######################################################################
# RBRV negative test helpers (set bad state → kindle → enforce)

zrbtcrv_rbrv_missing_sigil() {
  zrbtcrv_rbrv_baseline_conjure
  unset RBRV_SIGIL
  zrbrv_kindle
  zrbrv_enforce
}

zrbtcrv_rbrv_no_bind_image() {
  zrbtcrv_rbrv_baseline_bind
  unset RBRV_BIND_IMAGE
  zrbrv_kindle
  zrbrv_enforce
}

zrbtcrv_rbrv_unexpected_var() {
  zrbtcrv_rbrv_baseline_conjure
  export RBRV_BOGUS="foo"
  zrbrv_kindle
  zrbrv_enforce
}

zrbtcrv_rbrv_partial_conjure() {
  zrbtcrv_rbrv_baseline_conjure
  unset RBRV_CONJURE_PLATFORMS
  zrbrv_kindle
  zrbrv_enforce
}

######################################################################
# RBRV negative test cases

rbtcrv_rbrv_missing_sigil_tcase() {
  buto_trace "RBRV: missing RBRV_SIGIL must fail"
  buto_unit_expect_fatal zrbtcrv_rbrv_missing_sigil
}

rbtcrv_rbrv_no_bind_image_tcase() {
  buto_trace "RBRV: bind mode without RBRV_BIND_IMAGE must fail"
  buto_unit_expect_fatal zrbtcrv_rbrv_no_bind_image
}

rbtcrv_rbrv_unexpected_var_tcase() {
  buto_trace "RBRV: unexpected RBRV_BOGUS must fail scope sentinel"
  buto_unit_expect_fatal zrbtcrv_rbrv_unexpected_var
}

rbtcrv_rbrv_partial_conjure_tcase() {
  buto_trace "RBRV: conjure without CONJURE_PLATFORMS must fail"
  buto_unit_expect_fatal zrbtcrv_rbrv_partial_conjure
}

######################################################################
# RBRN baseline helpers — valid synthetic configurations

zrbtcrv_rbrn_baseline_disabled() {
  export RBRN_MONIKER="testrv"
  export RBRN_DESCRIPTION="Test nameplate"
  export RBRN_RUNTIME="docker"
  export RBRN_SENTRY_VESSEL="test-sentry"
  export RBRN_BOTTLE_VESSEL="test-bottle"
  export RBRN_SENTRY_CONSECRATION="c260101000000-r260101000000"
  export RBRN_BOTTLE_CONSECRATION="c260101000000-r260101000000"
  export RBRN_ENTRY_MODE="disabled"
  export RBRN_ENCLAVE_BASE_IP="10.200.0.0"
  export RBRN_ENCLAVE_NETMASK="24"
  export RBRN_ENCLAVE_SENTRY_IP="10.200.0.2"
  export RBRN_ENCLAVE_BOTTLE_IP="10.200.0.3"
  export RBRN_UPLINK_PORT_MIN="10000"
  export RBRN_UPLINK_DNS_MODE="disabled"
  export RBRN_UPLINK_ACCESS_MODE="disabled"
  export RBRN_DOCKER_VOLUME_MOUNTS=""
}

zrbtcrv_rbrn_baseline_enabled() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_ENTRY_MODE="enabled"
  export RBRN_ENTRY_PORT_WORKSTATION="8080"
  export RBRN_ENTRY_PORT_ENCLAVE="8888"
}

######################################################################
# RBRN negative test helpers (set bad state → kindle → enforce)

zrbtcrv_rbrn_missing_moniker() {
  zrbtcrv_rbrn_baseline_disabled
  unset RBRN_MONIKER
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_invalid_runtime() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_RUNTIME="invalid"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_invalid_entry_mode() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_ENTRY_MODE="bogus"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_invalid_dns_mode() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_UPLINK_DNS_MODE="bogus"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_invalid_access_mode() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_UPLINK_ACCESS_MODE="bogus"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_port_conflict() {
  zrbtcrv_rbrn_baseline_enabled
  export RBRN_ENTRY_PORT_WORKSTATION="10001"
  export RBRN_UPLINK_PORT_MIN="10000"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_unexpected_var() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_BOGUS="foo"
  zrbrn_kindle
  zrbrn_enforce
}

zrbtcrv_rbrn_bad_ip() {
  zrbtcrv_rbrn_baseline_disabled
  export RBRN_ENCLAVE_BASE_IP="not-an-ip"
  zrbrn_kindle
  zrbrn_enforce
}

######################################################################
# RBRN negative test cases

rbtcrv_rbrn_missing_moniker_tcase() {
  buto_trace "RBRN: missing RBRN_MONIKER must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_missing_moniker
}

rbtcrv_rbrn_invalid_runtime_tcase() {
  buto_trace "RBRN: invalid RBRN_RUNTIME must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_invalid_runtime
}

rbtcrv_rbrn_invalid_entry_mode_tcase() {
  buto_trace "RBRN: invalid RBRN_ENTRY_MODE must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_invalid_entry_mode
}

rbtcrv_rbrn_invalid_dns_mode_tcase() {
  buto_trace "RBRN: invalid RBRN_UPLINK_DNS_MODE must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_invalid_dns_mode
}

rbtcrv_rbrn_invalid_access_mode_tcase() {
  buto_trace "RBRN: invalid RBRN_UPLINK_ACCESS_MODE must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_invalid_access_mode
}

rbtcrv_rbrn_port_conflict_tcase() {
  buto_trace "RBRN: ENTRY_PORT_WORKSTATION >= UPLINK_PORT_MIN must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_port_conflict
}

rbtcrv_rbrn_unexpected_var_tcase() {
  buto_trace "RBRN: unexpected RBRN_BOGUS must fail scope sentinel"
  buto_unit_expect_fatal zrbtcrv_rbrn_unexpected_var
}

rbtcrv_rbrn_bad_ip_tcase() {
  buto_trace "RBRN: invalid RBRN_ENCLAVE_BASE_IP must fail"
  buto_unit_expect_fatal zrbtcrv_rbrn_bad_ip
}

######################################################################
# Positive test helpers (source real file → kindle → enforce)
#
# Each runs in a subshell via zbuto_invoke.  RBRR is NOT kindled in
# the fixture baste (so RBRR negative tests can override vars), so
# helpers that need RBRR kindle it themselves.

zrbtcrv_rbrr_validate_repo() {
  zrbrr_kindle
  zrbrr_enforce
}

zrbtcrv_rbrv_validate_vessel() {
  local z_sigil="${1}"
  source "${ZRBTCRV_VESSEL_DIR}/${z_sigil}/rbrv.env"
  zrbrv_kindle
  zrbrv_enforce
}

zrbtcrv_rbrn_validate_nameplate() {
  local z_moniker="${1}"
  source "${RBBC_dot_dir}/${RBCC_rbrn_prefix}${z_moniker}${RBCC_rbrn_ext}"
  zrbrn_kindle
  zrbrn_enforce
}

######################################################################
# Positive test cases

rbtcrv_rbrr_repo_tcase() {
  buto_trace "Validate real rbrr.env via kindle+enforce"
  buto_unit_expect_ok zrbtcrv_rbrr_validate_repo
}

rbtcrv_rbrv_all_vessels_tcase() {
  buto_trace "Validate all vessel rbrv.env files via kindle+enforce"
  local z_dirs=("${ZRBTCRV_VESSEL_DIR}"/*)
  local z_d=""
  for z_d in "${z_dirs[@]}"; do
    test -d "${z_d}" || continue
    test -f "${z_d}/rbrv.env" || continue
    local z_sigil="${z_d##*/}"
    buto_info "Validating vessel: ${z_sigil}"
    buto_unit_expect_ok zrbtcrv_rbrv_validate_vessel "${z_sigil}"
  done
}

rbtcrv_rbrn_all_nameplates_tcase() {
  buto_trace "Validate all nameplate rbrn_*.env files via kindle+enforce"
  local z_monikers
  z_monikers=$(rbrn_list_capture) || buto_fatal "No nameplates found"
  local z_moniker=""
  for z_moniker in ${z_monikers}; do
    buto_info "Validating nameplate: ${z_moniker}"
    buto_unit_expect_ok zrbtcrv_rbrn_validate_nameplate "${z_moniker}"
  done
}

# eof
