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
# RBTCNS - Nsproto security test cases for RBTB testbench

set -euo pipefail

######################################################################
# Test Cases - nsproto security tests
#
# Ported from rbt_testbench.sh test_nsproto_* functions
# Pattern: buto_unit_expect_ok = success expected, buto_unit_expect_fatal = failure expected
# Use _i variants for commands that read stdin (dig, traceroute, apt-get)

#--- Basic network verification ---

rbtcns_basic_dnsmasq_tcase() {
  # Verify dnsmasq is running on sentry
  buto_unit_expect_ok rbtb_exec_sentry ps aux
  rbtb_exec_sentry ps aux | grep -q dnsmasq || buto_fatal "dnsmasq not running on sentry"
}

rbtcns_basic_ping_sentry_tcase() {
  # Censer can ping sentry within enclave
  buto_unit_expect_ok rbtb_exec_censer ping "${RBRN_ENCLAVE_SENTRY_IP}" -c 2
}

rbtcns_basic_iptables_tcase() {
  # Verify RBM-INGRESS chain exists
  buto_unit_expect_ok rbtb_exec_sentry iptables -L RBM-INGRESS
}

#--- DNS allow/block tests ---

rbtcns_dns_allow_anthropic_tcase() {
  buto_unit_expect_ok rbtb_exec_bottle nslookup anthropic.com
}

rbtcns_dns_block_google_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle nslookup google.com
}

#--- TCP 443 connection tests ---

rbtcns_tcp443_allow_anthropic_tcase() {
  # Get anthropic IP from sentry (which can resolve anything)
  local z_ip
  z_ip=$(rbtb_exec_sentry_i dig +short anthropic.com | head -1)
  test -n "${z_ip}" || buto_fatal "Failed to resolve anthropic.com"
  buto_unit_expect_ok rbtb_exec_bottle nc -w 2 -zv "${z_ip}" 443
}

rbtcns_tcp443_block_google_tcase() {
  # Get google IP from sentry, then verify bottle cannot connect
  local z_ip
  z_ip=$(rbtb_exec_sentry_i dig +short google.com | head -1)
  test -n "${z_ip}" || buto_fatal "Failed to resolve google.com"
  buto_unit_expect_fatal rbtb_exec_bottle nc -w 2 -zv "${z_ip}" 443
}

#--- DNS protocol tests ---

rbtcns_dns_nonexist_tcase() {
  # Non-existent domain should fail with NXDOMAIN
  local z_output
  z_output=$(rbtb_exec_bottle nslookup nonexistentdomain123.test 2>&1 || true)
  echo "${z_output}" | grep -q NXDOMAIN || buto_fatal "Expected NXDOMAIN in output: ${z_output}"
}

rbtcns_dns_tcp_tcase() {
  # DNS over TCP should work for allowed domains
  buto_unit_expect_ok rbtb_exec_bottle_i dig +tcp anthropic.com
}

rbtcns_dns_notcp_tcase() {
  # DNS over UDP should work for allowed domains
  buto_unit_expect_ok rbtb_exec_bottle_i dig +notcp anthropic.com
}

#--- DNS security tests (block bypass attempts) ---

rbtcns_dns_block_direct_tcase() {
  # Cannot query external DNS directly
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @8.8.8.8 anthropic.com
  # Cannot connect to external DNS port
  buto_unit_expect_fatal rbtb_exec_bottle nc -w 2 -zv 8.8.8.8 53
}

rbtcns_dns_block_altport_tcase() {
  # Cannot use alternate DNS ports
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @8.8.8.8 -p 5353 example.com
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @8.8.8.8 -p 443 example.com
}

rbtcns_dns_block_cloudflare_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @1.1.1.1 example.com
}

rbtcns_dns_block_quad9_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @9.9.9.9 example.com
}

rbtcns_dns_block_zonetransfer_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @8.8.8.8 example.com AXFR
}

rbtcns_dns_block_ipv6_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @2001:4860:4860::8888 example.com
}

rbtcns_dns_block_multicast_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @224.0.0.251 -p 5353 example.local
}

rbtcns_dns_block_spoofing_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle_i dig @8.8.8.8 +nsid example.com -b 192.168.1.2
}

rbtcns_dns_block_tunneling_tcase() {
  buto_unit_expect_fatal rbtb_exec_bottle nc -z -w 1 8.8.8.8 53
}

#--- Package management test ---

rbtcns_block_packages_tcase() {
  # apt-get update should fail (cannot reach package repos)
  buto_unit_expect_fatal rbtb_exec_bottle_i timeout 5 apt-get -qq update
}

#--- ICMP tests ---

rbtcns_icmp_sentry_only_tcase() {
  # First hop should be sentry (podman) or blocked (Docker)
  # Both are acceptable - key is traffic routes through sentry
  local z_output
  z_output=$(rbtb_exec_bottle_i traceroute -I -m 1 8.8.8.8 2>&1)
  # Accept either sentry IP visible OR fully blocked (* * *)
  if echo "${z_output}" | grep -q "${RBRN_ENCLAVE_SENTRY_IP}"; then
    : # Sentry responded (podman behavior)
  elif echo "${z_output}" | grep -qE "^\s*1\s+\* \* \*"; then
    : # Blocked at first hop (Docker behavior - more restrictive)
  else
    buto_fatal "Unexpected traceroute output (expected sentry IP or * * *): ${z_output}"
  fi
}

rbtcns_icmp_block_beyond_tcase() {
  # Second hop should timeout (blocked)
  local z_output
  z_output=$(rbtb_exec_bottle_i traceroute -I -m 2 8.8.8.8 2>&1)
  echo "${z_output}" | grep -qE "^[[:space:]]*2[[:space:]]+\* \* \*" || \
    buto_fatal "Expected blocked second hop (* * *) in traceroute: ${z_output}"
}

# eof
