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
# RBTCAP - Access probe test cases for RBTB testbench
#
# These tests run BEFORE the ark-lifecycle suite as a fast smoke test
# (~30 seconds) for OAuth/credential issues that would otherwise surface
# only after the 4-minute ark-lifecycle suite begins.
#
# Regression tests for the rbgo_OAuth.sh stderr-capture fix (pace AfAAR).
#
# Delegates to rbap_AccessProbe.sh public functions for actual probe logic.

set -euo pipefail

######################################################################
# Test cases
#
# Each test case kindles the rbap module and calls the corresponding
# public probe function.  Parameters (iterations, delay) are set by
# the suite setup function in rbtb_testbench.sh via ZRBTCAP_* globals.

# rbtcap_jwt_governor_tcase - JWT SA access probe for Governor role
rbtcap_jwt_governor_tcase() {
  buto_info "JWT access probe: Governor"
  rbrr_load
  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbap_kindle

  rbap_jwt_sa_probe "governor" "${ZRBTCAP_ITERATIONS}" "${ZRBTCAP_DELAY_MS}"
  buto_success "Governor JWT access probe passed"
}

# rbtcap_jwt_director_tcase - JWT SA access probe for Director role
rbtcap_jwt_director_tcase() {
  buto_info "JWT access probe: Director"
  rbrr_load
  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbap_kindle

  rbap_jwt_sa_probe "director" "${ZRBTCAP_ITERATIONS}" "${ZRBTCAP_DELAY_MS}"
  buto_success "Director JWT access probe passed"
}

# rbtcap_jwt_retriever_tcase - JWT SA access probe for Retriever role
rbtcap_jwt_retriever_tcase() {
  buto_info "JWT access probe: Retriever"
  rbrr_load
  zrbgc_kindle
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbap_kindle

  rbap_jwt_sa_probe "retriever" "${ZRBTCAP_ITERATIONS}" "${ZRBTCAP_DELAY_MS}"
  buto_success "Retriever JWT access probe passed"
}

# rbtcap_payor_oauth_tcase - Payor OAuth access probe
rbtcap_payor_oauth_tcase() {
  buto_info "Payor OAuth access probe"
  rbrr_load
  zrbgc_kindle
  rbrp_load
  zrbgo_kindle
  zrbgu_kindle
  zrbgi_kindle
  zrbgp_kindle
  zrbap_kindle

  rbap_payor_oauth_probe "${ZRBTCAP_ITERATIONS}" "${ZRBTCAP_DELAY_MS}"
  buto_success "Payor OAuth access probe passed"
}

# eof
