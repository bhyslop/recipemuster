#!/usr/bin/env python3
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
# RBTIE - DNS exfiltration via subdomain encoding
#
# Attack surface: DNS allowlist permits queries to anthropic.com.
# This module tests whether data can be exfiltrated by encoding it
# as subdomains of the allowed domain (e.g., secret.anthropic.com).
#
# The sentry's dnsmasq should either block fabricated subdomains
# or the query should fail to resolve (NXDOMAIN). Either outcome
# means the exfiltration channel is closed.
#
# Category: dns
# Requires: scapy (for packet-level DNS construction)

import subprocess


def run():
    """Execute DNS subdomain exfiltration probe."""
    assertions = []

    # Assertion 1: fabricated subdomain of allowed domain should not resolve
    exfil_domain = "exfil-test-payload.anthropic.com"
    try:
        result = subprocess.run(
            ["nslookup", exfil_domain],
            capture_output=True, text=True, timeout=5
        )
        # nslookup returns 0 on success, nonzero on failure
        if result.returncode != 0:
            assertions.append({
                "name": "subdomain_nxdomain",
                "passed": True,
                "detail": f"nslookup {exfil_domain} failed as expected (NXDOMAIN or refused)",
            })
        else:
            # Resolved — check if it's a real IP or a sinkhole
            assertions.append({
                "name": "subdomain_nxdomain",
                "passed": False,
                "detail": f"nslookup {exfil_domain} resolved unexpectedly: {result.stdout.strip()}",
            })
    except subprocess.TimeoutExpired:
        assertions.append({
            "name": "subdomain_nxdomain",
            "passed": True,
            "detail": "nslookup timed out (query blocked by sentry)",
        })
    except Exception as e:
        assertions.append({
            "name": "subdomain_nxdomain",
            "passed": False,
            "detail": f"Unexpected error: {e}",
        })

    # Assertion 2: long encoded subdomain (simulating data payload)
    encoded_payload = "7365637265742d64617461" + ".anthropic.com"
    try:
        result = subprocess.run(
            ["nslookup", encoded_payload],
            capture_output=True, text=True, timeout=5
        )
        blocked = result.returncode != 0
        assertions.append({
            "name": "encoded_payload_blocked",
            "passed": blocked,
            "detail": f"Encoded subdomain {'blocked' if blocked else 'resolved'}: {encoded_payload}",
        })
    except subprocess.TimeoutExpired:
        assertions.append({
            "name": "encoded_payload_blocked",
            "passed": True,
            "detail": "Encoded subdomain query timed out (blocked)",
        })
    except Exception as e:
        assertions.append({
            "name": "encoded_payload_blocked",
            "passed": False,
            "detail": f"Unexpected error: {e}",
        })

    all_passed = all(a["passed"] for a in assertions)
    return {
        "verdict": "PASS" if all_passed else "FAIL",
        "detail": "DNS subdomain exfiltration channel is closed" if all_passed
                  else "DNS subdomain exfiltration channel may be open",
        "assertions": assertions,
    }
