# Copyright 2024 Scale Invariant, Inc.
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

# Main entry point
rbt_test_bottle_service_rule:                \
  ztest_info_rule                            \
  ztest_basic_network_rule                   \
  ztest_bottle_dns_allow_anthropic_rule      \
  ztest_bottle_dns_block_google_rule         \
  ztest_bottle_tcp443_allow_anthropic_rule   \
  ztest_bottle_tcp443_block_google_rule      \
  ztest_bottle_dns_nonexist_rule             \
  ztest_bottle_dns_tcp_rule                  \
  ztest_bottle_dns_notcp_rule                \
  ztest_bottle_dns_block_direct_rule         \
  ztest_bottle_dns_block_altport_rule        \
  ztest_bottle_dns_block_cloudflare_rule     \
  ztest_bottle_dns_block_quad9_rule          \
  ztest_bottle_dns_block_zonetransfer_rule   \
  ztest_bottle_dns_block_ipv6_rule           \
  ztest_bottle_dns_block_multicast_rule      \
  ztest_bottle_dns_block_spoofing_rule       \
  ztest_bottle_dns_block_tunneling_rule      \
  ztest_bottle_block_packages_rule           \
  ztest_bottle_icmp_sentry_only_rule         \
  ztest_bottle_icmp_block_beyond_rule        \
  # end-list
	$(MBC_PASS) "No errors seen."

# Information collection
ztest_info_rule:
	@echo "RBM_SENTRY_CONTAINER:  $(RBM_SENTRY_CONTAINER)"
	@echo "RBM_BOTTLE_CONTAINER:  $(RBM_BOTTLE_CONTAINER)"
	@echo "RBN_ENCLAVE_SENTRY_IP: $(RBN_ENCLAVE_SENTRY_IP)"
	@echo "RBV_TEMP_DIR:          $(RBV_TEMP_DIR)"

# Basic network setup verification - must run after info but before other tests
ztest_basic_network_rule: ztest_info_rule
	podman exec $(RBM_SENTRY_CONTAINER) ps aux | grep dnsmasq
	podman exec $(RBM_BOTTLE_CONTAINER) ping $(RBN_ENCLAVE_SENTRY_IP) -c 2
	podman exec $(RBM_SENTRY_CONTAINER) iptables -L RBM-INGRESS

# DNS resolution tests
ztest_bottle_dns_allow_anthropic_rule: ztest_basic_network_rule
	podman exec $(RBM_BOTTLE_CONTAINER) nslookup anthropic.com

ztest_bottle_dns_block_google_rule: ztest_basic_network_rule
	! podman exec $(RBM_BOTTLE_CONTAINER) nslookup google.com

# TCP connection tests
ztest_bottle_tcp443_allow_anthropic_rule: ztest_basic_network_rule
	@ANTHROPIC_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short anthropic.com | head -1) && \
	  podman exec $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$ANTHROPIC_IP 443

ztest_bottle_tcp443_block_google_rule: ztest_basic_network_rule
	@GOOGLE_IP=$$(podman exec $(RBM_SENTRY_CONTAINER) dig +short google.com | head -1) && \
	  ! podman exec $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv $$GOOGLE_IP 443

# DNS protocol tests
ztest_bottle_dns_nonexist_rule: ztest_basic_network_rule
	podman exec -i $(RBM_BOTTLE_CONTAINER) nslookup nonexistentdomain123.test | grep NXDOMAIN

ztest_bottle_dns_tcp_rule: ztest_basic_network_rule
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +tcp anthropic.com

ztest_bottle_dns_notcp_rule: ztest_basic_network_rule
	podman exec -i $(RBM_BOTTLE_CONTAINER) dig +notcp anthropic.com

# DNS security tests
ztest_bottle_dns_block_direct_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 anthropic.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) nc -w 2 -zv 8.8.8.8 53

ztest_bottle_dns_block_altport_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 5353 example.com
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 -p 443 example.com

ztest_bottle_dns_block_cloudflare_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @1.1.1.1 example.com

ztest_bottle_dns_block_quad9_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @9.9.9.9 example.com

ztest_bottle_dns_block_zonetransfer_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 example.com AXFR

ztest_bottle_dns_block_ipv6_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @2001:4860:4860::8888 example.com

ztest_bottle_dns_block_multicast_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @224.0.0.251%eth0 -p 5353 example.local

ztest_bottle_dns_block_spoofing_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) dig @8.8.8.8 +nsid example.com -b 192.168.1.2

ztest_bottle_dns_block_tunneling_rule: ztest_basic_network_rule
	! podman exec $(RBM_BOTTLE_CONTAINER) nc -z -w 1 8.8.8.8 53

# Package management test - runs independently after network setup
ztest_bottle_block_packages_rule: ztest_basic_network_rule
	! podman exec -i $(RBM_BOTTLE_CONTAINER) timeout 2 apt-get -qq update 2>&1 | grep -q "Could not resolve"

# ICMP tests
ztest_bottle_icmp_sentry_only_rule: ztest_basic_network_rule
	podman exec -i $(RBM_BOTTLE_CONTAINER) traceroute -I -m 1 8.8.8.8 2>&1 | grep -q "$(RBN_ENCLAVE_SENTRY_IP)"

ztest_bottle_icmp_block_beyond_rule: ztest_basic_network_rule
	podman exec -i $(RBM_BOTTLE_CONTAINER) traceroute -I -m 2 8.8.8.8 2>&1 | grep -q "^[[:space:]]*2[[:space:]]*\* \* \*"


# eof
