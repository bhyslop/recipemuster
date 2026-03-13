#!/bin/bash
# RBGJB Step 01: Derive consecration from inscribe + build timestamps
# Builder: gcr.io/cloud-builders/gcloud
#
# Dual-timestamp format: i20260224_153022-b20260224_160530
# - Inscribe timestamp (iYYYYMMDD_HHMMSS) baked at inscribe time via _RBGY_INSCRIBE_TIMESTAMP
# - Build timestamp (bYYYYMMDD_HHMMSS) derived at build time
# Multiple builds of same inscribed JSON sort together by inscribe prefix

set -euo pipefail

echo "Build strategy: ${ZRBF_BUILD_STRATEGY}"

CONSECRATION="${_RBGY_INSCRIBE_TIMESTAMP}-b$(date -u +%Y%m%d_%H%M%S)"
test -n "${CONSECRATION}" || { echo "CONSECRATION empty" >&2; exit 1; }
echo "${CONSECRATION}" > .consecration

# Expose consecration via Cloud Build step output mechanism
# Results appear in results.buildStepOutputs[0] (base64-encoded, max 50 bytes)
echo -n "${CONSECRATION}" > /builder/outputs/output
