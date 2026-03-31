#!/bin/sh
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
# RBJC - Censer initialization script
# Configures network routing through sentry for enclave isolation.
# Baked into sentry image at /opt/rbk/rbjc_censer.sh.
#
# Requires: RBRN_ENCLAVE_SENTRY_IP in container environment

set -e

echo "RBJC: Beginning censer setup"

echo "RBJC: Validate parameters"
: "${RBRN_ENCLAVE_SENTRY_IP:?}" && echo "RBJC: RBRN_ENCLAVE_SENTRY_IP = ${RBRN_ENCLAVE_SENTRY_IP}"

echo "RBJC: Configuring DNS to use sentry"
echo "nameserver ${RBRN_ENCLAVE_SENTRY_IP}" > /etc/resolv.conf || exit 10

echo "RBJC: Discovering enclave interface (single non-loopback interface expected)"
z_temp_file="/tmp/rbjc_iface_discovery.txt"
ip -o -4 addr show scope global > "${z_temp_file}" || exit 11
read z_num RBJC_ENCLAVE_IF z_rest < "${z_temp_file}"
rm -f "${z_temp_file}"
test -n "${RBJC_ENCLAVE_IF}" || { echo "RBJC: FATAL - No enclave interface found"; exit 11; }
echo "RBJC: Enclave interface = ${RBJC_ENCLAVE_IF}"

echo "RBJC: Flushing ARP entries"
ip link set ${RBJC_ENCLAVE_IF} down && ip link set ${RBJC_ENCLAVE_IF} up && ip -s -s neigh flush all || exit 20

echo "RBJC: Setting default route through sentry"
ip route add default via "${RBRN_ENCLAVE_SENTRY_IP}" || exit 30

echo "RBJC: Verifying default route"
ip route | grep -q "^default via ${RBRN_ENCLAVE_SENTRY_IP}" || { echo "RBJC: FATAL - default route not set"; exit 31; }

echo "RBJC: Signaling health"
touch /tmp/rbjch_healthy || exit 40

echo "RBJC: Censer setup complete, entering hold"
exec sleep infinity
