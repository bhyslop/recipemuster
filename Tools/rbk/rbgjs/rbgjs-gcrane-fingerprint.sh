#!/bin/bash
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
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
# RBGJS gcrane-fingerprint — read an upstream image's raw manifest, take its
# canonical sha256 digest, and derive the sanitized-origin glance fingerprint.
# The canonical digest is the sha256 of the RAW manifest bytes (what every tool
# reports); the fingerprint is the sanitized origin (`:` and `/` become `-`) plus
# the first 10 hex of that digest — the ANCHOR tag form. `gcrane
# manifest` streams the registry's stored manifest bytes verbatim (no
# re-serialization), so the digest is byte-identical to the canonical OCI
# digest. The first GAR copy is kind-specific (its
# destination tag is itself digest-derived) and stays in the step.
#   requires: ORIGIN    upstream image ref (e.g. docker.io/library/x:tag)
#             RAW_FILE  /workspace path to write the raw manifest into
#   provides: SHA          64-hex canonical digest (no algorithm prefix)
#             FINGERPRINT  "<sanitized-origin>-<first-10-hex>"
gcrane manifest "${ORIGIN}" > "${RAW_FILE}" \
  || { echo "FATAL: Failed to read upstream manifest: ${ORIGIN}" >&2; exit 1; }

# sha256sum (busybox coreutils, not openssl) — runs inside the gcrane:debug builder.
SHA=$(sha256sum "${RAW_FILE}" | cut -d' ' -f1)
test -n "${SHA}" || { echo "FATAL: Empty digest for ${ORIGIN}" >&2; exit 1; }

# Sanitize origin (: and / become -), append first 10 hex chars (legacy anchor form).
FINGERPRINT="$(printf '%s' "${ORIGIN}" | tr ':/' '--')-${SHA:0:10}"
