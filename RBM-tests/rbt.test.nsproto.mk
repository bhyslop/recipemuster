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
  ztest_pod_localhost_dns_rule               \
  ztest_pod_service_port_rule                \
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
	@echo "RBM_SENTRY_CONTAINER:   $(RBM_SENTRY_CONTAINER)"
	@echo "RBM_BOTTLE_CONTAINER:   $(RBM_BOTTLE_CONTAINER)"
	@echo "MBD_TEMP_DIR:           $(MBD_TEMP_DIR)"
	@echo "RBM_MACHINE:            $(RBM_MACHINE)"
	@test -n "$(RBM_SENTRY_CONTAINER)"   || (echo "Error: RBM_SENTRY_CONTAINER   must be set" && exit 1)
	@test -n "$(RBM_BOTTLE_CONTAINER)"   || (echo "Error: RBM_BOTTLE_CONTAINER   must be set" && exit 1)
	@test -n "$(MBD_TEMP_DIR)"           || (echo "Error: MBD_TEMP_DIR           must be set" && exit 1)
	@test -n "$(RBM_MACHINE)"            || (echo "Error: RBM_MACHINE            must be set" && exit 1)


# Basic network setup verification - must run after info but before other tests
ztest_basic_network_rule: ztest_info_rule
	$(MBT_PODMAN_EXEC_SENTRY) ps aux | grep dnsmasq
	$(MBT_PODMAN_EXEC_BOTTLE) nc -zv 127.0.0.1 53
	$(MBT_PODMAN_EXEC_SENTRY) iptables -L RBM-INGRESS

# Pod-specific network tests
ztest_pod_localhost_dns_rule: ztest_basic_network_rule
	$(MBT_PODMAN_EXEC_BOTTLE) dig @127.0.0.1 anthropic.com

ztest_pod_service_port_rule: ztest_basic_network_rule
	timeout 10 curl -s --connect-timeout 5 telnet://localhost:$(RBRN_ENTRY_PORT_WORKSTATION) > /dev/null  2>&1

# DNS resolution tests
ztest_bottle_dns_allow_anthropic_rule: ztest_basic_network_rule
	$(MBT_PODMAN_EXEC_BOTTLE) nslookup anthropic.com

ztest_bottle_dns_block_google_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE) nslookup google.com

# TCP connection tests
ztest_bottle_tcp443_allow_anthropic_rule: ztest_basic_network_rule
	@ANTHROPIC_IP=$$($(MBT_PODMAN_EXEC_SENTRY) dig +short anthropic.com | head -1) && \
	  $(MBT_PODMAN_EXEC_BOTTLE) nc -w 2 -zv $$ANTHROPIC_IP 443

ztest_bottle_tcp443_block_google_rule: ztest_basic_network_rule
	@GOOGLE_IP=$$($(MBT_PODMAN_EXEC_SENTRY) dig +short google.com | head -1) && \
	  ! $(MBT_PODMAN_EXEC_BOTTLE) nc -w 2 -zv $$GOOGLE_IP 443

# DNS protocol tests
ztest_bottle_dns_nonexist_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE) nslookup nonexistentdomain123.test >  $(MBD_TEMP_DIR)/dns_test_output.txt 2>&1
	cat              $(MBD_TEMP_DIR)/dns_test_output.txt
	grep -q NXDOMAIN $(MBD_TEMP_DIR)/dns_test_output.txt || exit 1

ztest_bottle_dns_tcp_rule: ztest_basic_network_rule
	$(MBT_PODMAN_EXEC_BOTTLE_I) dig +tcp anthropic.com

ztest_bottle_dns_notcp_rule: ztest_basic_network_rule
	$(MBT_PODMAN_EXEC_BOTTLE_I) dig +notcp anthropic.com

# DNS security tests
ztest_bottle_dns_block_direct_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 anthropic.com
	! $(MBT_PODMAN_EXEC_BOTTLE_I) nc -w 2 -zv 8.8.8.8 53

ztest_bottle_dns_block_altport_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 -p 5353 example.com
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 -p 443 example.com

ztest_bottle_dns_block_cloudflare_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @1.1.1.1 example.com

ztest_bottle_dns_block_quad9_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @9.9.9.9 example.com

ztest_bottle_dns_block_zonetransfer_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 example.com AXFR

ztest_bottle_dns_block_ipv6_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @2001:4860:4860::8888 example.com

ztest_bottle_dns_block_multicast_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @224.0.0.251%eth0 -p 5353 example.local

ztest_bottle_dns_block_spoofing_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) dig @8.8.8.8 +nsid example.com -b 192.168.1.2

ztest_bottle_dns_block_tunneling_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE) nc -z -w 1 8.8.8.8 53

# Package management test - runs independently after network setup
ztest_bottle_block_packages_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) timeout 2 apt-get -qq update 2>&1 | grep -q "Could not resolve"

# ICMP tests
ztest_bottle_icmp_sentry_only_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) ping -c 1 -W 1 8.8.8.8

ztest_bottle_icmp_block_beyond_rule: ztest_basic_network_rule
	! $(MBT_PODMAN_EXEC_BOTTLE_I) traceroute -I -m 1 -w 1 8.8.8.8


# eof
