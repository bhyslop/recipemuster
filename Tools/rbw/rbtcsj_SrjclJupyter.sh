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
# RBTCSJ - Srjcl Jupyter test cases for RBTB testbench
#
# Ported from RBM-tests/rbt.test.srjcl.mk
# Pattern: Verify Jupyter server health, then run Python WebSocket test

set -euo pipefail

# Test container image for Python networking tests
RBTCSJ_TEST_IMAGE="ghcr.io/bhyslop/recipemuster:rbtest_python_networking.20250215__171409"

######################################################################
# Test Cases

rbtcsj_jupyter_running_tcase() {
  buto_trace "Verifying Jupyter process is running in bottle"
  # Verify Jupyter process is running in bottle
  buto_unit_expect_ok rbtb_exec_bottle ps aux
  rbtb_exec_bottle ps aux | grep -q jupyter || buto_fatal "jupyter not running in bottle"
}

rbtcsj_jupyter_connectivity_tcase() {
  buto_trace "Testing basic HTTP connectivity to Jupyter from host"
  # Test basic HTTP connectivity to Jupyter from host
  # Uses curl with browser-like headers to access JupyterLab
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab"
  local z_output
  z_output=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "User-Agent: Mozilla/5.0" \
    -H "Accept: text/html,application/xhtml+xml" \
    --connect-timeout 5 --max-time 10 \
    "${z_url}" 2>&1) || true
  test "${z_output}" = "200" || buto_fatal "Expected HTTP 200 from Jupyter, got: ${z_output}"
}

rbtcsj_websocket_kernel_tcase() {
  buto_trace "Running full Python test (WebSocket, session creation, kernel execution)"
  # Run the full Python test (WebSocket, session creation, kernel execution)
  # This test container runs on host network and connects to Jupyter
  local z_test_script="${RBTB_SCRIPT_DIR}/../../RBM-tests/rbt.test.srjcl.py"
  test -f "${z_test_script}" || buto_fatal "Test script not found: ${z_test_script}"

  # Run Python test in dedicated container with networking dependencies
  # Note: Python script uses RBRN_ENTRY_PORT_WORKSTATION env var
  buto_unit_expect_ok ${ZRBOB_RUNTIME} run --rm -i \
    --network host \
    -e RBRN_ENTRY_PORT_WORKSTATION="${RBRN_ENTRY_PORT_WORKSTATION}" \
    "${RBTCSJ_TEST_IMAGE}" \
    python3 - < "${z_test_script}"
}

# eof
