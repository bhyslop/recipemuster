#!/bin/bash
# RBGJB Step 01: Derive hallmark from inscribe + build timestamps
# Builder: gcr.io/cloud-builders/gcloud
#
# Hallmark format: [cbg]YYMMDDHHMMSS-rYYMMDDHHMMSS
# - Mode prefix: c (conjure), b (bind), g (graft)
# - Inscribe timestamp (cYYMMDDHHMMSS) baked at inscribe time via _RBGY_INSCRIBE_TIMESTAMP
# - Realized timestamp (rYYMMDDHHMMSS) derived at build time (when image lands in GAR)
# Multiple builds of same inscribed JSON sort together by inscribe prefix

set -euo pipefail

echo "Build strategy: ${ZRBF_BUILD_STRATEGY}"

HALLMARK="${_RBGY_INSCRIBE_TIMESTAMP}-r$(date -u +%y%m%d%H%M%S)"
test -n "${HALLMARK}" || { echo "HALLMARK empty" >&2; exit 1; }
echo "${HALLMARK}" > .hallmark

# Expose hallmark via Cloud Build step output mechanism
# Results appear in results.buildStepOutputs[0] (base64-encoded, max 50 bytes)
echo -n "${HALLMARK}" > /builder/outputs/output
