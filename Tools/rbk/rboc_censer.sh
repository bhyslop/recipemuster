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
# RBOC - Censer initialization script
# Configures network routing through sentry for enclave isolation.
# Mounted transitionally from host; baked into sentry image in future pace.
#
# Requires: RBRN_ENCLAVE_SENTRY_IP in container environment

set -e

echo "RBOC: Beginning censer setup"

echo "RBOC: Validate parameters"
: "${RBRN_ENCLAVE_SENTRY_IP:?}" && echo "RBOC: RBRN_ENCLAVE_SENTRY_IP = ${RBRN_ENCLAVE_SENTRY_IP}"

echo "RBOC: Configuring DNS to use sentry"
echo "nameserver ${RBRN_ENCLAVE_SENTRY_IP}" > /etc/resolv.conf || exit 10

echo "RBOC: Flushing ARP entries"
ip link set eth0 down && ip link set eth0 up && ip -s -s neigh flush all || exit 20

echo "RBOC: Setting default route through sentry"
ip route add default via "${RBRN_ENCLAVE_SENTRY_IP}" || exit 30

echo "RBOC: Verifying default route"
ip route | grep -q "^default via ${RBRN_ENCLAVE_SENTRY_IP}" || { echo "RBOC: FATAL - default route not set"; exit 31; }

echo "RBOC: Signaling health"
touch /tmp/rboc_healthy || exit 40

echo "RBOC: Censer setup complete, entering hold"
exec sleep infinity
