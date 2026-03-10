#!/bin/bash
# RBGJV Step 02: Discover per-platform digests, verify SLSA provenance, compose summary
# Builder: gcloud (via RBRG_GCLOUD_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL,
#                _RBGV_CONSECRATION, _RBGV_SOURCE_URI,
#                _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256
#
# Note: Uses python3 for JSON parsing (ships with gcloud image).
# No jq dependency — avoids apt-get calls for long-term robustness.

set -euo pipefail
echo "=== Discover digests and verify provenance ==="

pyjson() { python3 -c "import json,sys; $1"; }

FULL_IMAGE="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
IMAGE_TAG="${_RBGV_CONSECRATION}-image"
BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"

# Fetch manifest list via registry API
TOKEN=$(gcloud auth print-access-token)
MANIFEST_URL="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}/manifests/${IMAGE_TAG}"
echo "Fetching manifest list: ${IMAGE_TAG}"
curl -sS \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json" \
  "${MANIFEST_URL}" > /workspace/manifest_list.json

DIGEST_COUNT=$(pyjson "d=json.load(open('/workspace/manifest_list.json')); print(len(d['manifests']))")
echo "Found ${DIGEST_COUNT} platform entries"
test "${DIGEST_COUNT}" -gt 0 || { echo "FATAL: no platform entries in manifest list" >&2; exit 1; }

# Extract platform info as lines of "digest arch variant"
pyjson "
d=json.load(open('/workspace/manifest_list.json'))
for m in d['manifests']:
    p=m.get('platform',{})
    print(m['digest'], p.get('architecture',''), p.get('variant',''))
" > /workspace/platform_entries.txt

while read -r DIGEST ARCH VARIANT; do
  if [ -n "${VARIANT}" ]; then
    PLAT_SUFFIX="${ARCH}${VARIANT}"
  else
    PLAT_SUFFIX="${ARCH}"
  fi
  FULL_REF="${FULL_IMAGE}@${DIGEST}"
  echo "Verifying ${PLAT_SUFFIX} (${DIGEST})..."

  echo "  Fetching provenance for ${PLAT_SUFFIX}..."
  gcloud artifacts docker images describe "${FULL_REF}" \
    --format json --show-provenance \
    > "/workspace/provenance-${PLAT_SUFFIX}.json"

  echo "  Running slsa-verifier for ${PLAT_SUFFIX}..."
  /workspace/slsa-verifier verify-image "${FULL_REF}" \
    --provenance-path "/workspace/provenance-${PLAT_SUFFIX}.json" \
    --source-uri "${_RBGV_SOURCE_URI}" \
    --builder-id="${BUILDER_ID}" \
    --print-provenance \
    > "/workspace/verify-${PLAT_SUFFIX}.json"
  echo "  Platform ${PLAT_SUFFIX} verified"
done < /workspace/platform_entries.txt

echo "All ${DIGEST_COUNT} platforms verified"

echo "Composing vouch summary..."
python3 -c "
import json, glob, os
verdicts = []
for f in sorted(glob.glob('/workspace/verify-*.json')):
    plat = os.path.basename(f).replace('verify-','').replace('.json','')
    verdicts.append({'platform': plat, 'verdict': 'pass'})
summary = {
    'consecration': '${_RBGV_CONSECRATION}',
    'vessel': '${_RBGV_VESSEL}',
    'verifier': {
        'url': '${_RBGV_VERIFIER_URL}',
        'sha256': '${_RBGV_VERIFIER_SHA256}'
    },
    'platforms': verdicts
}
with open('/workspace/vouch_summary.json','w') as out:
    json.dump(summary, out, indent=2)
"
echo "Vouch summary composed"
