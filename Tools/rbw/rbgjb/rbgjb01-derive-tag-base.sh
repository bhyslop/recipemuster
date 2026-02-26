#!/bin/bash
# RBGJB Step 01: Derive TAG_BASE from inscribe + build timestamps
# Builder: gcr.io/cloud-builders/gcloud
#
# Dual-timestamp format: i20260224_153022-b20260224_160530
# - Inscribe timestamp (iYYYYMMDD_HHMMSS) baked at inscribe time via _RBGY_INSCRIBE_TIMESTAMP
# - Build timestamp (bYYYYMMDD_HHMMSS) derived at build time
# Multiple builds of same inscribed JSON sort together by inscribe prefix

set -euo pipefail

TAG_BASE="${_RBGY_INSCRIBE_TIMESTAMP}-b$(date -u +%Y%m%d_%H%M%S)"
test -n "${TAG_BASE}" || { echo "TAG_BASE empty" >&2; exit 1; }
echo "${TAG_BASE}" > .tag_base
