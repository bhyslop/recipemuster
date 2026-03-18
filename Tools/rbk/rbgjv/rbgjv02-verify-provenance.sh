#!/bin/bash
# RBGJV Step 02: Mode-aware verification and vouch summary composition
# Branches on _RBGV_VESSEL_MODE: conjure (SLSA), bind (digest-pin), graft (stamp)
# Writes /workspace/vouch_platforms.txt for step 03.

set -euo pipefail
echo "=== Mode-aware verification (${_RBGV_VESSEL_MODE}) ==="

pyjson() { python3 -c "import json,sys; $1"; }

FULL_IMAGE="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
IMAGE_TAG="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}"
REGISTRY_BASE="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
ACCEPT="application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json"

TOKEN=$(gcloud auth print-access-token)

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
  *manifest.list*|*image.index*) IS_INDEX="true" ;;
esac

if [ "${IS_INDEX}" = "true" ]; then
  echo "Multi-platform manifest detected"
  printf '%s' "${MANIFEST}" | pyjson "
d=json.load(sys.stdin)
platforms=[]
for m in d['manifests']:
    p=m.get('platform',{})
    if p.get('os','')=='unknown' and p.get('architecture','')=='unknown': continue
    plat=p.get('os','')+'/'+p.get('architecture','')
    v=p.get('variant','')
    if v: plat+='/'+v
    platforms.append(plat)
if not platforms: print('FATAL: No platforms',file=sys.stderr); sys.exit(1)
print(','.join(platforms))
" > /workspace/vouch_platforms.txt
else
  echo "Single manifest detected"
  CONFIG_DIGEST=$(printf '%s' "${MANIFEST}" | pyjson "print(json.load(sys.stdin)['config']['digest'])")
  test -n "${CONFIG_DIGEST}" || { echo "FATAL: No config digest" >&2; exit 1; }
  CONFIG=$(curl -sf -H "Authorization: Bearer ${TOKEN}" "${REGISTRY_BASE}/blobs/${CONFIG_DIGEST}") \
    || { echo "FATAL: Failed to fetch config blob" >&2; exit 1; }
  PLATFORM=$(printf '%s' "${CONFIG}" | pyjson "
d=json.load(sys.stdin)
p=d.get('os','linux')+'/'+d.get('architecture','amd64')
v=d.get('variant','')
if v: p+='/'+v
print(p)
")
  echo "${PLATFORM}" > /workspace/vouch_platforms.txt
fi
echo "Platforms for vouch: $(cat /workspace/vouch_platforms.txt)"

case "${_RBGV_VESSEL_MODE}" in

  conjure)
    BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"
    if [ "${IS_INDEX}" != "true" ]; then
      echo "FATAL: conjure image must be multi-platform" >&2; exit 1
    fi

    printf '%s' "${MANIFEST}" > /workspace/manifest_list.json
    pyjson "
d=json.load(open('/workspace/manifest_list.json'))
for m in d['manifests']:
    p=m.get('platform',{})
    if p.get('os','')=='unknown' and p.get('architecture','')=='unknown': continue
    print(m['digest'],p.get('architecture',''),p.get('variant',''))
" > /workspace/platform_entries.txt

    DIGEST_COUNT=$(wc -l < /workspace/platform_entries.txt | tr -d ' ')
    echo "Found ${DIGEST_COUNT} platform entries"
    test "${DIGEST_COUNT}" -gt 0 || { echo "FATAL: no platform entries" >&2; exit 1; }

    if [ -n "${_RBGV_SOURCE_URI}" ]; then
      VERIFY_METHOD="slsa-verifier"
      echo "Using slsa-verifier (source URI present)"
    else
      VERIFY_METHOD="provenance-direct"
      echo "Using direct provenance verification (builds.create path)"
    fi

    while read -r DIGEST ARCH VARIANT; do
      PLAT_SUFFIX="${ARCH}${VARIANT}"
      FULL_REF="${FULL_IMAGE}@${DIGEST}"
      echo "Verifying ${PLAT_SUFFIX} (${DIGEST})..."
      echo "  Fetching provenance..."
      gcloud artifacts docker images describe "${FULL_REF}" \
        --format json --show-provenance \
        > "/workspace/provenance-${PLAT_SUFFIX}.json"
      if [ "${VERIFY_METHOD}" = "slsa-verifier" ]; then
        /workspace/slsa-verifier verify-image "${FULL_REF}" \
          --provenance-path "/workspace/provenance-${PLAT_SUFFIX}.json" \
          --source-uri "${_RBGV_SOURCE_URI}" \
          --builder-id="${BUILDER_ID}" \
          --print-provenance \
          > "/workspace/verify-${PLAT_SUFFIX}.json"
      else
        python3 /workspace/direct_verify.py \
          "/workspace/provenance-${PLAT_SUFFIX}.json" \
          "/workspace/verify-${PLAT_SUFFIX}.json" "${BUILDER_ID}"
      fi
      echo "  ${PLAT_SUFFIX} verified"
    done < /workspace/platform_entries.txt

    echo "All ${DIGEST_COUNT} platforms verified"
    python3 -c "
import json,glob,os
verdicts=[{'platform':os.path.basename(f).replace('verify-','').replace('.json',''),'verdict':'pass'} for f in sorted(glob.glob('/workspace/verify-*.json'))]
json.dump({'consecration':'${_RBGV_CONSECRATION}','vessel':'${_RBGV_VESSEL}','vessel_mode':'conjure','verify_method':'${VERIFY_METHOD}','verifier':{'url':'${_RBGV_VERIFIER_URL}','sha256':'${_RBGV_VERIFIER_SHA256}'},'platforms':verdicts},open('/workspace/vouch_summary.json','w'),indent=2)
"
    echo "Vouch summary composed"
    ;;

  bind)
    echo "Verifying digest-pin for bind vessel"
    HEADERS=$(curl -sI \
      -H "Authorization: Bearer ${TOKEN}" \
      -H "Accept: ${ACCEPT}" \
      "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
      || { echo "FATAL: HEAD request failed" >&2; exit 1; }
    ACTUAL_DIGEST=$(printf '%s' "${HEADERS}" | grep -i "docker-content-digest" | sed 's/^[^:]*: *//' | tr -d '\r\n')
    test -n "${ACTUAL_DIGEST}" || { echo "FATAL: No Docker-Content-Digest" >&2; exit 1; }
    echo "Actual digest: ${ACTUAL_DIGEST}"
    BIND_SOURCE="${_RBGV_BIND_SOURCE}"
    PINNED_DIGEST="${BIND_SOURCE#*@}"
    test -n "${PINNED_DIGEST}" || { echo "FATAL: no digest in BIND_SOURCE" >&2; exit 1; }
    test "${PINNED_DIGEST}" != "${BIND_SOURCE}" || { echo "FATAL: no @ in BIND_SOURCE" >&2; exit 1; }
    echo "Pinned digest: ${PINNED_DIGEST}"
    if [ "${ACTUAL_DIGEST}" = "${PINNED_DIGEST}" ]; then
      VERDICT="DIGEST_PIN_VERIFIED"
      echo "Digest match confirmed"
    else
      echo "FATAL: Digest mismatch — GAR: ${ACTUAL_DIGEST}  Pin: ${PINNED_DIGEST}" >&2; exit 1
    fi
    python3 -c "
import json
json.dump({'consecration':'${_RBGV_CONSECRATION}','vessel':'${_RBGV_VESSEL}','vessel_mode':'bind','verification':{'method':'digest-pin','bind_source':'${_RBGV_BIND_SOURCE}','actual_digest':'${ACTUAL_DIGEST}','pinned_digest':'${PINNED_DIGEST}','verdict':'${VERDICT}'}},open('/workspace/vouch_summary.json','w'),indent=2)
"
    echo "Vouch summary composed"
    ;;

  graft)
    echo "Graft mode — stamping GRAFTED"
    python3 -c "
import json
json.dump({'consecration':'${_RBGV_CONSECRATION}','vessel':'${_RBGV_VESSEL}','vessel_mode':'graft','verification':{'method':'none','graft_source':'${_RBGV_GRAFT_SOURCE}','verdict':'GRAFTED'}},open('/workspace/vouch_summary.json','w'),indent=2)
"
    echo "Vouch summary composed"
    ;;

  *) echo "FATAL: Unknown vessel mode: ${_RBGV_VESSEL_MODE}" >&2; exit 1 ;;
esac
