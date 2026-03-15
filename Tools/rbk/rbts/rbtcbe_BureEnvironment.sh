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
# RBTCBE - BURE ambient environment test cases for RBTB testbench
#
# Exercises BURE tweak mechanism enrollment: positive tests for valid
# tweak combinations, negative tests for length violations and scope
# sentinel.  All tests are pure local — no GCP, no containers, no network.

set -euo pipefail

######################################################################
# BURE tweak positive helpers (set env → kindle → enforce)

zrbtcbe_tweak_empty() {
  zbure_kindle
  zbure_enforce
}

zrbtcbe_tweak_both_set() {
  export BURE_TWEAK_NAME="graft_image"
  export BURE_TWEAK_VALUE="us-docker.pkg.dev/proj/repo/img:latest"
  zbure_kindle
  zbure_enforce
}

zrbtcbe_tweak_name_only() {
  export BURE_TWEAK_NAME="graft_image"
  zbure_kindle
  zbure_enforce
}

zrbtcbe_tweak_value_only() {
  export BURE_TWEAK_VALUE="some-override-value"
  zbure_kindle
  zbure_enforce
}

######################################################################
# BURE tweak negative helpers (set bad state → kindle → enforce)

zrbtcbe_tweak_name_too_long() {
  export BURE_TWEAK_NAME="$(printf 'x%.0s' {1..65})"
  zbure_kindle
  zbure_enforce
}

zrbtcbe_tweak_value_too_long() {
  export BURE_TWEAK_VALUE="$(printf 'x%.0s' {1..257})"
  zbure_kindle
  zbure_enforce
}

zrbtcbe_unexpected_var() {
  export BURE_BOGUS="foo"
  zbure_kindle
  zbure_enforce
}

######################################################################
# Positive test cases

rbtcbe_tweak_empty_tcase() {
  buto_trace "BURE: empty tweaks (default) must pass"
  buto_unit_expect_ok zrbtcbe_tweak_empty
}

rbtcbe_tweak_both_set_tcase() {
  buto_trace "BURE: both tweak name and value set must pass"
  buto_unit_expect_ok zrbtcbe_tweak_both_set
}

rbtcbe_tweak_name_only_tcase() {
  buto_trace "BURE: tweak name without value must pass"
  buto_unit_expect_ok zrbtcbe_tweak_name_only
}

rbtcbe_tweak_value_only_tcase() {
  buto_trace "BURE: tweak value without name must pass"
  buto_unit_expect_ok zrbtcbe_tweak_value_only
}

######################################################################
# Negative test cases

rbtcbe_tweak_name_too_long_tcase() {
  buto_trace "BURE: tweak name exceeding 64 chars must fail"
  buto_unit_expect_fatal zrbtcbe_tweak_name_too_long
}

rbtcbe_tweak_value_too_long_tcase() {
  buto_trace "BURE: tweak value exceeding 256 chars must fail"
  buto_unit_expect_fatal zrbtcbe_tweak_value_too_long
}

rbtcbe_unexpected_var_tcase() {
  buto_trace "BURE: unexpected BURE_BOGUS must fail scope sentinel"
  buto_unit_expect_fatal zrbtcbe_unexpected_var
}

# eof
