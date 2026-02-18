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
# RBTCPL - Pluml PlantUML test cases for RBTB testbench
#
# Ported from RBM-tests/rbt.test.pluml.mk
# Pattern: Test PlantUML server HTTP endpoints

set -euo pipefail

######################################################################
# Test Cases

rbtcpl_text_rendering_tcase() {
  buto_trace "Testing PlantUML text rendering endpoint with known diagram hash"
  # Test PlantUML text rendering endpoint with known diagram hash
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40"
  local z_output
  z_output=$(curl -s "${z_url}" 2>&1) || buto_fatal "curl failed: ${z_output}"
  echo "${z_output}" | grep -q "Bob"         || buto_fatal "Expected 'Bob' in response"
  echo "${z_output}" | grep -q "Alice"       || buto_fatal "Expected 'Alice' in response"
  echo "${z_output}" | grep -q "hello there" || buto_fatal "Expected 'hello there' in response"
  echo "${z_output}" | grep -q "boo"         || buto_fatal "Expected 'boo' in response"
}

rbtcpl_local_diagram_tcase() {
  buto_trace "Testing PlantUML server with local diagram POST to /txt/uml"
  # Test PlantUML server with local diagram POST to /txt/uml
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/uml"
  local z_diagram="@startuml\nBob -> Alice: hello there\nAlice --> Bob: boo\n@enduml"
  local z_output
  z_output=$(echo -e "${z_diagram}" | curl -s --data-binary @- "${z_url}" 2>&1) || buto_fatal "curl POST failed: ${z_output}"
  echo "${z_output}" | grep -q "Bob"         || buto_fatal "Expected 'Bob' in response"
  echo "${z_output}" | grep -q "Alice"       || buto_fatal "Expected 'Alice' in response"
  echo "${z_output}" | grep -q "hello there" || buto_fatal "Expected 'hello there' in response"
  echo "${z_output}" | grep -q "boo"         || buto_fatal "Expected 'boo' in response"
}

rbtcpl_http_headers_tcase() {
  buto_trace "Testing server handles basic HTTP headers"
  # Test server handles basic HTTP headers
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40"
  local z_status
  z_status=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "User-Agent: Mozilla/5.0" \
    -H "Accept: text/plain" \
    --connect-timeout 5 --max-time 10 \
    "${z_url}" 2>&1) || true
  test "${z_status}" = "200" || buto_fatal "Expected HTTP 200, got: ${z_status}"
}

rbtcpl_invalid_hash_tcase() {
  buto_trace "Testing server response with invalid diagram hash returns no content"
  # Test server response with invalid diagram hash returns no content
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/invalid_hash"
  local z_output
  z_output=$(curl -s "${z_url}" 2>&1) || true
  local z_count
  z_count=$(echo "${z_output}" | grep -c "Bob" || true)
  test "${z_count}" -eq 0 || buto_fatal "Expected no 'Bob' in invalid hash response"
}

rbtcpl_malformed_diagram_tcase() {
  buto_trace "Testing server response with malformed diagram returns no valid content"
  # Test server response with malformed diagram returns no valid content
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/uml"
  local z_output
  z_output=$(echo "invalid uml content" | curl -s --data-binary @- "${z_url}" 2>&1) || true
  local z_count
  z_count=$(echo "${z_output}" | grep -c "Bob" || true)
  test "${z_count}" -eq 0 || buto_fatal "Expected no 'Bob' in malformed diagram response"
}

# eof
