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
# RBT Testbench - Recipe Bottle test execution
#
# Commands:
#   rbt-to  Run test suite for a nameplate (e.g., rbt-to nsproto)

set -euo pipefail

# Get script directory
RBT_SCRIPT_DIR="${BASH_SOURCE[0]%/*}"

# Source dependencies
source "${RBT_SCRIPT_DIR}/../buk/buc_command.sh"
source "${RBT_SCRIPT_DIR}/../buk/buv_validation.sh"
source "${RBT_SCRIPT_DIR}/../buk/but_test.sh"
source "${RBT_SCRIPT_DIR}/rbrn_regime.sh"
source "${RBT_SCRIPT_DIR}/rbrr_regime.sh"
source "${RBT_SCRIPT_DIR}/rbob_bottle.sh"

# Show filename on each displayed line
buc_context "${0##*/}"

######################################################################
# Helper Functions

# Verbose output if BURD_VERBOSE is set
rbt_show() {
  test "${BURD_VERBOSE:-0}" != "1" || echo "RBTSHOW: $*"
}

# Load nameplate configuration by moniker and kindle RBOB
# Usage: rbt_load_nameplate <moniker>
rbt_load_nameplate() {
  local z_moniker="${1:-}"
  test -n "${z_moniker}" || buc_die "rbt_load_nameplate: moniker argument required"

  local z_nameplate_file="${RBT_SCRIPT_DIR}/rbrn_${z_moniker}.env"
  test -f "${z_nameplate_file}" || buc_die "Nameplate not found: ${z_nameplate_file}"

  rbt_show "Loading nameplate: ${z_nameplate_file}"
  source "${z_nameplate_file}" || buc_die "Failed to source nameplate: ${z_nameplate_file}"

  rbt_show "Kindling nameplate regime"
  zrbrn_kindle
  zrbrn_validate_fields

  rbt_show "Nameplate loaded: RBRN_MONIKER=${RBRN_MONIKER}, RBRN_RUNTIME=${RBRN_RUNTIME}"

  # Load RBRR (repository regime)
  rbt_show "Loading RBRR"
  local z_rbrr_file="${RBT_SCRIPT_DIR}/../../rbrr_RecipeBottleRegimeRepo.sh"
  test -f "${z_rbrr_file}" || buc_die "RBRR config not found: ${z_rbrr_file}"
  source "${z_rbrr_file}" || buc_die "Failed to source RBRR config: ${z_rbrr_file}"
  zrbrr_kindle

  # Kindle RBOB (validates RBRN and RBRR are ready, sets container names)
  rbt_show "Kindling RBOB"
  zrbob_kindle
}

######################################################################
# Container Exec Helpers
#
# Two variants per container:
#   rbt_exec_*   - Simple exec (no -i), for commands that just output and exit
#   rbt_exec_*_i - With -i flag, for commands that may read stdin (dig, traceroute, apt-get)

rbt_exec_sentry() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_SENTRY}" "$@"
}

rbt_exec_sentry_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_SENTRY}" "$@"
}

rbt_exec_censer() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_CENSER}" "$@"
}

rbt_exec_censer_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_CENSER}" "$@"
}

rbt_exec_bottle() {
  ${ZRBOB_RUNTIME} exec "${ZRBOB_BOTTLE}" "$@"
}

rbt_exec_bottle_i() {
  ${ZRBOB_RUNTIME} exec -i "${ZRBOB_BOTTLE}" "$@"
}

######################################################################
# Test Cases - nsproto security tests
#
# Ported from RBM-tests/rbt.test.nsproto.mk
# Pattern: but_unit_expect_ok = success expected, but_unit_expect_fatal = failure expected
# Use _i variants for commands that read stdin (dig, traceroute, apt-get)

#--- Basic network verification ---

test_nsproto_basic_dnsmasq() {
  # Verify dnsmasq is running on sentry
  but_unit_expect_ok rbt_exec_sentry ps aux
  rbt_exec_sentry ps aux | grep -q dnsmasq || but_fatal "dnsmasq not running on sentry"
}

test_nsproto_basic_ping_sentry() {
  # Censer can ping sentry within enclave
  but_unit_expect_ok rbt_exec_censer ping "${RBRN_ENCLAVE_SENTRY_IP}" -c 2
}

test_nsproto_basic_iptables() {
  # Verify RBM-INGRESS chain exists
  but_unit_expect_ok rbt_exec_sentry iptables -L RBM-INGRESS
}

#--- DNS allow/block tests ---

test_nsproto_dns_allow_anthropic() {
  but_unit_expect_ok rbt_exec_bottle nslookup anthropic.com
}

test_nsproto_dns_block_google() {
  but_unit_expect_fatal rbt_exec_bottle nslookup google.com
}

#--- TCP 443 connection tests ---

test_nsproto_tcp443_allow_anthropic() {
  # Get anthropic IP from sentry (which can resolve anything)
  local z_ip
  z_ip=$(rbt_exec_sentry_i dig +short anthropic.com | head -1)
  test -n "${z_ip}" || but_fatal "Failed to resolve anthropic.com"
  but_unit_expect_ok rbt_exec_bottle nc -w 2 -zv "${z_ip}" 443
}

test_nsproto_tcp443_block_google() {
  # Get google IP from sentry, then verify bottle cannot connect
  local z_ip
  z_ip=$(rbt_exec_sentry_i dig +short google.com | head -1)
  test -n "${z_ip}" || but_fatal "Failed to resolve google.com"
  but_unit_expect_fatal rbt_exec_bottle nc -w 2 -zv "${z_ip}" 443
}

#--- DNS protocol tests ---

test_nsproto_dns_nonexist() {
  # Non-existent domain should fail with NXDOMAIN
  local z_output
  z_output=$(rbt_exec_bottle nslookup nonexistentdomain123.test 2>&1 || true)
  echo "${z_output}" | grep -q NXDOMAIN || but_fatal "Expected NXDOMAIN in output: ${z_output}"
}

test_nsproto_dns_tcp() {
  # DNS over TCP should work for allowed domains
  but_unit_expect_ok rbt_exec_bottle_i dig +tcp anthropic.com
}

test_nsproto_dns_notcp() {
  # DNS over UDP should work for allowed domains
  but_unit_expect_ok rbt_exec_bottle_i dig +notcp anthropic.com
}

#--- DNS security tests (block bypass attempts) ---

test_nsproto_dns_block_direct() {
  # Cannot query external DNS directly
  but_unit_expect_fatal rbt_exec_bottle_i dig @8.8.8.8 anthropic.com
  # Cannot connect to external DNS port
  but_unit_expect_fatal rbt_exec_bottle nc -w 2 -zv 8.8.8.8 53
}

test_nsproto_dns_block_altport() {
  # Cannot use alternate DNS ports
  but_unit_expect_fatal rbt_exec_bottle_i dig @8.8.8.8 -p 5353 example.com
  but_unit_expect_fatal rbt_exec_bottle_i dig @8.8.8.8 -p 443 example.com
}

test_nsproto_dns_block_cloudflare() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @1.1.1.1 example.com
}

test_nsproto_dns_block_quad9() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @9.9.9.9 example.com
}

test_nsproto_dns_block_zonetransfer() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @8.8.8.8 example.com AXFR
}

test_nsproto_dns_block_ipv6() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @2001:4860:4860::8888 example.com
}

test_nsproto_dns_block_multicast() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @224.0.0.251 -p 5353 example.local
}

test_nsproto_dns_block_spoofing() {
  but_unit_expect_fatal rbt_exec_bottle_i dig @8.8.8.8 +nsid example.com -b 192.168.1.2
}

test_nsproto_dns_block_tunneling() {
  but_unit_expect_fatal rbt_exec_bottle nc -z -w 1 8.8.8.8 53
}

#--- Package management test ---

test_nsproto_block_packages() {
  # apt-get update should fail (cannot reach package repos)
  but_unit_expect_fatal rbt_exec_bottle_i timeout 5 apt-get -qq update
}

#--- ICMP tests ---

test_nsproto_icmp_sentry_only() {
  # First hop should be sentry (podman) or blocked (Docker)
  # Both are acceptable - key is traffic routes through sentry
  local z_output
  z_output=$(rbt_exec_bottle_i traceroute -I -m 1 8.8.8.8 2>&1)
  # Accept either sentry IP visible OR fully blocked (* * *)
  if echo "${z_output}" | grep -q "${RBRN_ENCLAVE_SENTRY_IP}"; then
    : # Sentry responded (podman behavior)
  elif echo "${z_output}" | grep -qE "^\s*1\s+\* \* \*"; then
    : # Blocked at first hop (Docker behavior - more restrictive)
  else
    but_fatal "Unexpected traceroute output (expected sentry IP or * * *): ${z_output}"
  fi
}

test_nsproto_icmp_block_beyond() {
  # Second hop should timeout (blocked)
  local z_output
  z_output=$(rbt_exec_bottle_i traceroute -I -m 2 8.8.8.8 2>&1)
  echo "${z_output}" | grep -qE "^[[:space:]]*2[[:space:]]+\* \* \*" || \
    but_fatal "Expected blocked second hop (* * *) in traceroute: ${z_output}"
}

######################################################################
# Test Cases - srjcl Jupyter tests
#
# Ported from RBM-tests/rbt.test.srjcl.mk
# Pattern: Verify Jupyter server health, then run Python WebSocket test

# Test container image for Python networking tests
RBT_SRJCL_TEST_IMAGE="ghcr.io/bhyslop/recipemuster:rbtest_python_networking.20250215__171409"

test_srjcl_jupyter_running() {
  # Verify Jupyter process is running in bottle
  but_unit_expect_ok rbt_exec_bottle ps aux
  rbt_exec_bottle ps aux | grep -q jupyter || but_fatal "jupyter not running in bottle"
}

test_srjcl_jupyter_connectivity() {
  # Test basic HTTP connectivity to Jupyter from host
  # Uses curl with browser-like headers to access JupyterLab
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/lab"
  local z_output
  z_output=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "User-Agent: Mozilla/5.0" \
    -H "Accept: text/html,application/xhtml+xml" \
    --connect-timeout 5 --max-time 10 \
    "${z_url}" 2>&1) || true
  test "${z_output}" = "200" || but_fatal "Expected HTTP 200 from Jupyter, got: ${z_output}"
}

test_srjcl_websocket_kernel() {
  # Run the full Python test (WebSocket, session creation, kernel execution)
  # This test container runs on host network and connects to Jupyter
  local z_test_script="${RBT_SCRIPT_DIR}/../../RBM-tests/rbt.test.srjcl.py"
  test -f "${z_test_script}" || but_fatal "Test script not found: ${z_test_script}"

  # Run Python test in dedicated container with networking dependencies
  # Note: Python script uses RBRN_ENTRY_PORT_WORKSTATION env var
  but_unit_expect_ok ${ZRBOB_RUNTIME} run --rm -i \
    --network host \
    -e RBRN_ENTRY_PORT_WORKSTATION="${RBRN_ENTRY_PORT_WORKSTATION}" \
    "${RBT_SRJCL_TEST_IMAGE}" \
    python3 - < "${z_test_script}"
}

######################################################################
# Test Cases - pluml PlantUML tests
#
# Ported from RBM-tests/rbt.test.pluml.mk
# Pattern: Test PlantUML server HTTP endpoints

test_pluml_text_rendering() {
  # Test PlantUML text rendering endpoint with known diagram hash
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40"
  local z_output
  z_output=$(curl -s "${z_url}" 2>&1) || but_fatal "curl failed: ${z_output}"
  echo "${z_output}" | grep -q "Bob"         || but_fatal "Expected 'Bob' in response"
  echo "${z_output}" | grep -q "Alice"       || but_fatal "Expected 'Alice' in response"
  echo "${z_output}" | grep -q "hello there" || but_fatal "Expected 'hello there' in response"
  echo "${z_output}" | grep -q "boo"         || but_fatal "Expected 'boo' in response"
}

test_pluml_local_diagram() {
  # Test PlantUML server with local diagram POST to /txt/uml
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/uml"
  local z_diagram="@startuml\nBob -> Alice: hello there\nAlice --> Bob: boo\n@enduml"
  local z_output
  z_output=$(echo -e "${z_diagram}" | curl -s --data-binary @- "${z_url}" 2>&1) || but_fatal "curl POST failed: ${z_output}"
  echo "${z_output}" | grep -q "Bob"         || but_fatal "Expected 'Bob' in response"
  echo "${z_output}" | grep -q "Alice"       || but_fatal "Expected 'Alice' in response"
  echo "${z_output}" | grep -q "hello there" || but_fatal "Expected 'hello there' in response"
  echo "${z_output}" | grep -q "boo"         || but_fatal "Expected 'boo' in response"
}

test_pluml_http_headers() {
  # Test server handles basic HTTP headers
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/SyfFKj2rKt3CoKnELR1Io4ZDoSbNACb8BKhbWeZf0cMTyfEi59Boym40"
  local z_status
  z_status=$(curl -s -o /dev/null -w "%{http_code}" \
    -H "User-Agent: Mozilla/5.0" \
    -H "Accept: text/plain" \
    --connect-timeout 5 --max-time 10 \
    "${z_url}" 2>&1) || true
  test "${z_status}" = "200" || but_fatal "Expected HTTP 200, got: ${z_status}"
}

test_pluml_invalid_hash() {
  # Test server response with invalid diagram hash returns no content
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/invalid_hash"
  local z_output
  z_output=$(curl -s "${z_url}" 2>&1) || true
  local z_count
  z_count=$(echo "${z_output}" | grep -c "Bob" || true)
  test "${z_count}" -eq 0 || but_fatal "Expected no 'Bob' in invalid hash response"
}

test_pluml_malformed_diagram() {
  # Test server response with malformed diagram returns no valid content
  local z_url="http://localhost:${RBRN_ENTRY_PORT_WORKSTATION}/txt/uml"
  local z_output
  z_output=$(echo "invalid uml content" | curl -s --data-binary @- "${z_url}" 2>&1) || true
  local z_count
  z_count=$(echo "${z_output}" | grep -c "Bob" || true)
  test "${z_count}" -eq 0 || but_fatal "Expected no 'Bob' in malformed diagram response"
}

######################################################################
# Test Suites

rbt_suite_nsproto() {
  local z_single_test="${1:-}"
  buc_step "Running nsproto security test suite${z_single_test:+ (single: ${z_single_test})}"
  local z_test_dir="${BURD_TEMP_DIR}/tests"
  mkdir -p "${z_test_dir}"
  but_execute "${z_test_dir}" "test_nsproto_" "${z_single_test}"
}

rbt_suite_srjcl() {
  local z_single_test="${1:-}"
  buc_step "Running srjcl Jupyter test suite${z_single_test:+ (single: ${z_single_test})}"
  local z_test_dir="${BURD_TEMP_DIR}/tests"
  mkdir -p "${z_test_dir}"
  but_execute "${z_test_dir}" "test_srjcl_" "${z_single_test}"
}

rbt_suite_pluml() {
  local z_single_test="${1:-}"
  buc_step "Running pluml PlantUML test suite${z_single_test:+ (single: ${z_single_test})}"
  local z_test_dir="${BURD_TEMP_DIR}/tests"
  mkdir -p "${z_test_dir}"
  but_execute "${z_test_dir}" "test_pluml_" "${z_single_test}"
}

######################################################################
# Routing

rbt_route() {
  local z_command="${1:-}"
  shift || true
  local z_moniker="${1:-}"

  rbt_show "Routing command: ${z_command} with moniker: ${z_moniker}"

  # Verify BUD environment
  test -n "${BURD_TEMP_DIR:-}" || buc_die "BURD_TEMP_DIR not set - must be called from BUD"
  test -n "${BURD_NOW_STAMP:-}" || buc_die "BURD_NOW_STAMP not set - must be called from BUD"

  case "${z_command}" in
    rbt-to)
      test -n "${z_moniker}" || buc_die "rbt-to requires moniker argument"
      shift || true
      local z_single_test="${1:-}"
      rbt_load_nameplate "${z_moniker}"
      case "${z_moniker}" in
        nsproto) rbt_suite_nsproto "${z_single_test}" ;;
        srjcl)   rbt_suite_srjcl "${z_single_test}" ;;
        pluml)   rbt_suite_pluml "${z_single_test}" ;;
        *)       buc_die "Unknown test suite: ${z_moniker}" ;;
      esac
      ;;
    *) buc_die "Unknown command: ${z_command}" ;;
  esac
}

rbt_main() {
  local z_command="${1:-}"
  shift || true

  test -n "${z_command}" || buc_die "No command specified"

  rbt_route "${z_command}" "$@"
}

rbt_main "$@"

# eof
