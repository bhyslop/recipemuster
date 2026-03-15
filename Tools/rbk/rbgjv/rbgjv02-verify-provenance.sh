#!/bin/bash
# RBGJV Step 02: Mode-aware verification and vouch summary composition
# Builder: gcloud (via RBRG_GCLOUD_IMAGE_REF)
# Entrypoint: bash
# Substitutions: _RBGV_GAR_HOST, _RBGV_GAR_PATH, _RBGV_VESSEL,
#                _RBGV_CONSECRATION, _RBGV_VESSEL_MODE,
#                _RBGV_SOURCE_URI, _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256,
#                _RBGV_BIND_SOURCE, _RBGV_GRAFT_SOURCE,
#                _RBGV_ARK_SUFFIX_IMAGE
#
# Branches on _RBGV_VESSEL_MODE:
#   conjure: SLSA provenance verification via slsa-verifier
#   bind: digest-pin comparison against upstream reference
#   graft: GRAFTED stamp (no verification)
#
# Writes /workspace/vouch_platforms.txt for step 03 (platform discovery).
#
# Note: Uses python3 for JSON parsing (ships with gcloud image).
# No jq dependency — avoids apt-get calls for long-term robustness.

set -euo pipefail
echo "=== Mode-aware verification (${_RBGV_VESSEL_MODE}) ==="

pyjson() { python3 -c "import json,sys; $1"; }

FULL_IMAGE="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
IMAGE_TAG="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}"
REGISTRY_BASE="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
ACCEPT="application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json"

TOKEN=$(gcloud auth print-access-token)

# --- Platform discovery (all modes) ---
# Fetch -image manifest to discover platforms. Result written to /workspace/vouch_platforms.txt
# for step 03 (assemble-push-vouch). Pattern matches rbgja01-discover-platforms.sh.
echo "Discovering platforms from ${IMAGE_TAG}..."
MANIFEST=$(curl -sf \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: ${ACCEPT}" \
  "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
  || { echo "FATAL: Failed to fetch manifest for ${IMAGE_TAG}" >&2; exit 1; }

MEDIA_TYPE=$(printf '%s' "${MANIFEST}" | pyjson "print(json.load(sys.stdin).get('mediaType',''))")
echo "Manifest media type: ${MEDIA_TYPE}"

IS_INDEX="false"
case "${MEDIA_TYPE}" in
  *manifest.list*|*image.index*)
    IS_INDEX="true" ;;
esac

if [ "${IS_INDEX}" = "true" ]; then
  echo "Multi-platform manifest detected"
  # Filter out attestation manifests (platform unknown/unknown) — these are
  # SLSA provenance/SBOM entries, not runnable images.
  # See: https://docs.docker.com/build/metadata/attestations/attestation-storage/
  printf '%s' "${MANIFEST}" | pyjson "
d=json.load(sys.stdin)
platforms = []
filtered = 0
for m in d['manifests']:
    p=m.get('platform',{})
    if p.get('os','') == 'unknown' and p.get('architecture','') == 'unknown':
        filtered += 1
        continue
    plat = p.get('os','') + '/' + p.get('architecture','')
    v = p.get('variant','')
    if v: plat += '/' + v
    platforms.append(plat)
if filtered:
    print(f'Filtered {filtered} attestation manifest(s) (platform unknown/unknown)', file=sys.stderr)
if not platforms:
    print('FATAL: No runnable platforms after filtering attestation manifests', file=sys.stderr)
    sys.exit(1)
print(','.join(platforms))
" > /workspace/vouch_platforms.txt
else
  echo "Single manifest detected"
  CONFIG_DIGEST=$(printf '%s' "${MANIFEST}" | pyjson "print(json.load(sys.stdin)['config']['digest'])")
  test -n "${CONFIG_DIGEST}" || { echo "FATAL: No config digest in manifest" >&2; exit 1; }
  CONFIG=$(curl -sf \
    -H "Authorization: Bearer ${TOKEN}" \
    "${REGISTRY_BASE}/blobs/${CONFIG_DIGEST}") \
    || { echo "FATAL: Failed to fetch config blob" >&2; exit 1; }
  PLATFORM=$(printf '%s' "${CONFIG}" | pyjson "
d=json.load(sys.stdin)
p = d.get('os','linux') + '/' + d.get('architecture','amd64')
v = d.get('variant','')
if v: p += '/' + v
print(p)
")
  echo "${PLATFORM}" > /workspace/vouch_platforms.txt
fi
echo "Platforms for vouch: $(cat /workspace/vouch_platforms.txt)"

# --- Mode-specific verification ---
case "${_RBGV_VESSEL_MODE}" in

  conjure)
    BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"

    # Use already-fetched manifest for platform iteration
    if [ "${IS_INDEX}" != "true" ]; then
      echo "FATAL: conjure image must be multi-platform (got single manifest)" >&2
      exit 1
    fi

    # Write manifest to file for python extraction (filter attestation manifests)
    printf '%s' "${MANIFEST}" > /workspace/manifest_list.json

    pyjson "
d=json.load(open('/workspace/manifest_list.json'))
entries = []
for m in d['manifests']:
    p=m.get('platform',{})
    if p.get('os','') == 'unknown' and p.get('architecture','') == 'unknown':
        continue
    entries.append((m['digest'], p.get('architecture',''), p.get('variant','')))
for digest, arch, variant in entries:
    print(digest, arch, variant)
" > /workspace/platform_entries.txt

    DIGEST_COUNT=$(wc -l < /workspace/platform_entries.txt | tr -d ' ')
    echo "Found ${DIGEST_COUNT} platform entries (after filtering attestation manifests)"
    test "${DIGEST_COUNT}" -gt 0 || { echo "FATAL: no platform entries after filtering" >&2; exit 1; }

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
    'vessel_mode': 'conjure',
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
    ;;

  bind)
    echo "Verifying digest-pin for bind vessel"

    # HEAD request for -image to get Docker-Content-Digest
    HEADERS=$(curl -sI \
      -H "Authorization: Bearer ${TOKEN}" \
      -H "Accept: ${ACCEPT}" \
      "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
      || { echo "FATAL: HEAD request failed for ${IMAGE_TAG}" >&2; exit 1; }

    ACTUAL_DIGEST=$(printf '%s' "${HEADERS}" | grep -i "docker-content-digest" | sed 's/^[^:]*: *//' | tr -d '\r\n')
    test -n "${ACTUAL_DIGEST}" || { echo "FATAL: Docker-Content-Digest header not found" >&2; exit 1; }
    echo "Actual digest: ${ACTUAL_DIGEST}"

    # Extract pinned digest from _RBGV_BIND_SOURCE (the part after @).
    # Cloud Build substitution cannot handle ${VAR#pattern} expansion, so we
    # capture the substituted value into a local variable first.
    BIND_SOURCE="${_RBGV_BIND_SOURCE}"
    PINNED_DIGEST="${BIND_SOURCE#*@}"
    test -n "${PINNED_DIGEST}" || { echo "FATAL: no digest in _RBGV_BIND_SOURCE" >&2; exit 1; }
    test "${PINNED_DIGEST}" != "${BIND_SOURCE}" || { echo "FATAL: no @ delimiter in _RBGV_BIND_SOURCE" >&2; exit 1; }
    echo "Pinned digest: ${PINNED_DIGEST}"

    # Compare
    if [ "${ACTUAL_DIGEST}" = "${PINNED_DIGEST}" ]; then
      VERDICT="DIGEST_PIN_VERIFIED"
      echo "Digest match confirmed"
    else
      echo "FATAL: Digest mismatch — GAR: ${ACTUAL_DIGEST}  Pin: ${PINNED_DIGEST}" >&2
      exit 1
    fi

    echo "Composing vouch summary..."
    python3 -c "
import json
summary = {
    'consecration': '${_RBGV_CONSECRATION}',
    'vessel': '${_RBGV_VESSEL}',
    'vessel_mode': 'bind',
    'verification': {
        'method': 'digest-pin',
        'bind_source': '${_RBGV_BIND_SOURCE}',
        'actual_digest': '${ACTUAL_DIGEST}',
        'pinned_digest': '${PINNED_DIGEST}',
        'verdict': '${VERDICT}'
    }
}
with open('/workspace/vouch_summary.json','w') as out:
    json.dump(summary, out, indent=2)
"
    echo "Vouch summary composed"
    ;;

  graft)
    echo "Graft mode — no verification, stamping GRAFTED"

    echo "Composing vouch summary..."
    python3 -c "
import json
summary = {
    'consecration': '${_RBGV_CONSECRATION}',
    'vessel': '${_RBGV_VESSEL}',
    'vessel_mode': 'graft',
    'verification': {
        'method': 'none',
        'graft_source': '${_RBGV_GRAFT_SOURCE}',
        'verdict': 'GRAFTED'
    }
}
with open('/workspace/vouch_summary.json','w') as out:
    json.dump(summary, out, indent=2)
"
    echo "Vouch summary composed"
    ;;

  *)
    echo "FATAL: Unknown vessel mode: ${_RBGV_VESSEL_MODE}" >&2
    exit 1
    ;;
esac
