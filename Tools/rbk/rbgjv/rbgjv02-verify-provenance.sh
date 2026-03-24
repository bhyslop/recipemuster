#!/bin/bash
# RBGJV Step 02: Mode-aware verification and vouch summary composition
# conjure: DSSE envelope signature verification (jq + openssl)
# bind: digest-pin comparison | graft: GRAFTED stamp
# Writes /workspace/vouch_platforms.txt for step 03.

set -euo pipefail
echo "=== Mode-aware verification (${_RBGV_VESSEL_MODE}) ==="

# Use workspace jq (installed by step 01 from alpine — gcloud image lacks jq)
JQ=/workspace/bin/jq
test -x "${JQ}" || { echo "FATAL: jq not found at ${JQ} — step 01 must run first" >&2; exit 1; }

FULL_IMAGE="${_RBGV_GAR_HOST}/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
IMAGE_TAG="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}"
REGISTRY_BASE="https://${_RBGV_GAR_HOST}/v2/${_RBGV_GAR_PATH}/${_RBGV_VESSEL}"
ACCEPT="application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json"
TOKEN=$(gcloud auth print-access-token)

MANIFEST=$(curl -sf \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: ${ACCEPT}" \
  "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") \
  || { echo "FATAL: Failed to fetch manifest for ${IMAGE_TAG}" >&2; exit 1; }

MEDIA_TYPE=$(printf '%s' "${MANIFEST}" | ${JQ} -r '.mediaType // ""')
IS_INDEX="false"
case "${MEDIA_TYPE}" in *manifest.list*|*image.index*) IS_INDEX="true" ;; esac

if [ "${IS_INDEX}" = "true" ]; then
  printf '%s' "${MANIFEST}" | ${JQ} -r '[.manifests[] | select((.platform.os != "unknown") or (.platform.architecture != "unknown")) | .platform.os + "/" + .platform.architecture + (if .platform.variant then "/" + .platform.variant else "" end)] | join(",")' > /workspace/vouch_platforms.txt
  test -s /workspace/vouch_platforms.txt || { echo "FATAL: No platforms" >&2; exit 1; }
else
  CONFIG_DIGEST=$(printf '%s' "${MANIFEST}" | ${JQ} -r '.config.digest')
  test -n "${CONFIG_DIGEST}" || { echo "FATAL: No config digest" >&2; exit 1; }
  CONFIG=$(curl -sf -H "Authorization: Bearer ${TOKEN}" "${REGISTRY_BASE}/blobs/${CONFIG_DIGEST}") \
    || { echo "FATAL: Failed to fetch config blob" >&2; exit 1; }
  printf '%s' "${CONFIG}" | ${JQ} -r '.os + "/" + .architecture + (if .variant then "/" + .variant else "" end)' > /workspace/vouch_platforms.txt
fi
echo "Platforms: $(cat /workspace/vouch_platforms.txt)"

case "${_RBGV_VESSEL_MODE}" in

  conjure)
    BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"
    EXPECTED_KEYID="projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1"

    if [ "${IS_INDEX}" = "true" ]; then
      printf '%s' "${MANIFEST}" | ${JQ} -r '.manifests[] | select((.platform.os != "unknown") or (.platform.architecture != "unknown")) | [.digest, .platform.architecture, (.platform.variant // "")] | join(" ")' > /workspace/platform_entries.txt
    else
      ARCH=$(printf '%s' "${CONFIG}" | ${JQ} -r '.architecture')
      VARIANT=$(printf '%s' "${CONFIG}" | ${JQ} -r '.variant // ""')
      PS="${ARCH}${VARIANT}"
      AT="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}-${PS}"
      AH=$(curl -sI -H "Authorization: Bearer ${TOKEN}" -H "Accept: ${ACCEPT}" "${REGISTRY_BASE}/manifests/${AT}") \
        || { echo "FATAL: HEAD for ${AT} failed" >&2; exit 1; }
      AD=$(printf '%s' "${AH}" | grep -i "docker-content-digest" | sed 's/^[^:]*: *//' | tr -d '\r\n')
      test -n "${AD}" || { echo "FATAL: No digest for ${AT}" >&2; exit 1; }
      echo "${AD} ${ARCH} ${VARIANT}" > /workspace/platform_entries.txt
    fi

    DC=$(wc -l < /workspace/platform_entries.txt | tr -d ' ')
    echo "Verifying ${DC} platform(s) via DSSE"
    test "${DC}" -gt 0 || { echo "FATAL: no platform entries" >&2; exit 1; }

    while read -r DIGEST ARCH VARIANT; do
      PS="${ARCH}${VARIANT}"
      FR="${FULL_IMAGE}@${DIGEST}"
      echo "  ${PS}: fetching provenance..."
      gcloud artifacts docker images describe "${FR}" --format json --show-provenance > "/workspace/prov-${PS}.json"

      ${JQ} '.provenance_summary.provenance[] | select(.noteName | test("intoto_slsa_v1")) | .envelope' "/workspace/prov-${PS}.json" > "/workspace/env-${PS}.json"
      test -s "/workspace/env-${PS}.json" || { echo "FATAL: No v1.0 envelope for ${PS}" >&2; exit 1; }

      AK=$(${JQ} -r '.signatures[0].keyid' "/workspace/env-${PS}.json")
      test "${AK}" = "${EXPECTED_KEYID}" || { echo "FATAL: keyid mismatch: ${AK}" >&2; exit 1; }

      ${JQ} -j '.payload' "/workspace/env-${PS}.json" | tr '_-' '/+' | base64 -d > "/workspace/pl-${PS}.raw"
      ${JQ} -j '.signatures[0].sig' "/workspace/env-${PS}.json" | tr '_-' '/+' | base64 -d > "/workspace/sig-${PS}.bin"

      PT="application/vnd.in-toto+json"
      PL=$(wc -c < "/workspace/pl-${PS}.raw" | tr -d ' ')
      printf "DSSEv1 %d %s %d " "${#PT}" "${PT}" "${PL}" > "/workspace/pae-${PS}.bin"
      cat "/workspace/pl-${PS}.raw" >> "/workspace/pae-${PS}.bin"

      openssl dgst -sha256 -verify /workspace/keys/google-hosted-worker.pub \
        -signature "/workspace/sig-${PS}.bin" "/workspace/pae-${PS}.bin" \
        || { echo "FATAL: DSSE verify FAILED for ${PS}" >&2; exit 1; }

      AB=$(${JQ} -r '.predicate.runDetails.builder.id' "/workspace/pl-${PS}.raw")
      test "${AB}" = "${BUILDER_ID}" || { echo "FATAL: builder mismatch: ${AB}" >&2; exit 1; }

      BT=$(${JQ} -r '.predicate.buildDefinition.buildType' "/workspace/pl-${PS}.raw")
      SD=$(${JQ} -r '.subject[0].digest.sha256' "/workspace/pl-${PS}.raw")
      ${JQ} -n --arg v "dsse-envelope-v1" --arg b "${AB}" --arg t "${BT}" --arg s "${SD}" --arg k "${AK}" \
        '{verifier:$v,builder_id:$b,build_type:$t,subject_digest:$s,keyid:$k,verdict:"pass"}' \
        > "/workspace/verify-${PS}.json"
      echo "  ${PS}: DSSE verified OK"
    done < /workspace/platform_entries.txt

    echo "All ${DC} platforms verified"
    ${JQ} -n --arg c "${_RBGV_CONSECRATION}" --arg v "${_RBGV_VESSEL}" \
      '{consecration:$c,vessel:$v,vessel_mode:"conjure",verify_method:"dsse-envelope",base_images:[],platforms:[]}' \
      > /workspace/vouch_summary.json
    for f in /workspace/verify-*.json; do
      P=$(basename "${f}" .json); P="${P#verify-}"
      ${JQ} --arg p "${P}" --slurpfile e "${f}" '.platforms += [($e[0] + {platform:$p})]' \
        /workspace/vouch_summary.json > /workspace/vs_tmp.json
      mv /workspace/vs_tmp.json /workspace/vouch_summary.json
    done
    # Record base image provenance (anchored GAR refs or upstream pass-through)
    if [ -n "${_RBGV_IMAGE_1}" ]; then
      ${JQ} --arg ref "${_RBGV_IMAGE_1}" --arg prov "${_RBGV_IMAGE_1_PROVENANCE}" \
        '.base_images += [{slot:1,ref:$ref,provenance:$prov}]' \
        /workspace/vouch_summary.json > /workspace/vs_tmp.json
      mv /workspace/vs_tmp.json /workspace/vouch_summary.json
    fi
    if [ -n "${_RBGV_IMAGE_2}" ]; then
      ${JQ} --arg ref "${_RBGV_IMAGE_2}" --arg prov "${_RBGV_IMAGE_2_PROVENANCE}" \
        '.base_images += [{slot:2,ref:$ref,provenance:$prov}]' \
        /workspace/vouch_summary.json > /workspace/vs_tmp.json
      mv /workspace/vs_tmp.json /workspace/vouch_summary.json
    fi
    if [ -n "${_RBGV_IMAGE_3}" ]; then
      ${JQ} --arg ref "${_RBGV_IMAGE_3}" --arg prov "${_RBGV_IMAGE_3_PROVENANCE}" \
        '.base_images += [{slot:3,ref:$ref,provenance:$prov}]' \
        /workspace/vouch_summary.json > /workspace/vs_tmp.json
      mv /workspace/vs_tmp.json /workspace/vouch_summary.json
    fi
    echo "Vouch summary composed"
    ;;

  bind)
    HEADERS=$(curl -sI -H "Authorization: Bearer ${TOKEN}" -H "Accept: ${ACCEPT}" \
      "${REGISTRY_BASE}/manifests/${IMAGE_TAG}") || { echo "FATAL: HEAD failed" >&2; exit 1; }
    ACTUAL_DIGEST=$(printf '%s' "${HEADERS}" | grep -i "docker-content-digest" | sed 's/^[^:]*: *//' | tr -d '\r\n')
    test -n "${ACTUAL_DIGEST}" || { echo "FATAL: No Docker-Content-Digest" >&2; exit 1; }
    BS="${_RBGV_BIND_SOURCE}"; PD="${BS#*@}"
    test -n "${PD}" || { echo "FATAL: no digest in BIND_SOURCE" >&2; exit 1; }
    test "${PD}" != "${BS}" || { echo "FATAL: no @ in BIND_SOURCE" >&2; exit 1; }
    test "${ACTUAL_DIGEST}" = "${PD}" || { echo "FATAL: Digest mismatch — GAR: ${ACTUAL_DIGEST}  Pin: ${PD}" >&2; exit 1; }
    echo "Digest pin verified: ${ACTUAL_DIGEST}"
    ${JQ} -n --arg c "${_RBGV_CONSECRATION}" --arg v "${_RBGV_VESSEL}" --arg bs "${BS}" \
      --arg ad "${ACTUAL_DIGEST}" --arg pd "${PD}" \
      '{consecration:$c,vessel:$v,vessel_mode:"bind",verification:{method:"digest-pin",bind_source:$bs,actual_digest:$ad,pinned_digest:$pd,verdict:"DIGEST_PIN_VERIFIED"}}' \
      > /workspace/vouch_summary.json
    echo "Vouch summary composed"
    ;;

  graft)
    echo "Graft mode — stamping GRAFTED"
    ${JQ} -n --arg c "${_RBGV_CONSECRATION}" --arg v "${_RBGV_VESSEL}" --arg gs "${_RBGV_GRAFT_SOURCE}" \
      '{consecration:$c,vessel:$v,vessel_mode:"graft",verification:{method:"none",graft_source:$gs,verdict:"GRAFTED"}}' \
      > /workspace/vouch_summary.json
    echo "Vouch summary composed"
    ;;

  *) echo "FATAL: Unknown vessel mode: ${_RBGV_VESSEL_MODE}" >&2; exit 1 ;;
esac
