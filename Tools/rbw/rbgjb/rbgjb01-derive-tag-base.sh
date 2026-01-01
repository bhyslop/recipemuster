#!/bin/bash
# RBGJB Step 01: Derive TAG_BASE from current UTC time
# Builder: gcr.io/cloud-builders/gcloud

set -euo pipefail

# Generate TAG_BASE from current UTC time (compact format)
TAG_BASE="$(date -u +%Y%m%dT%H%M%SZ)"
test -n "${TAG_BASE}" || (echo "TAG_BASE empty" >&2; exit 1)
echo "${TAG_BASE}" > .tag_base
