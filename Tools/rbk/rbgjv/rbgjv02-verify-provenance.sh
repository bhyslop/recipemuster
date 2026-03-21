#!/bin/bash
# RBGJV Step 02: Mode-aware verification and vouch summary composition
# Branches on _RBGV_VESSEL_MODE: conjure (DSSE), bind (digest-pin), graft (stamp)
# Writes /workspace/vouch_platforms.txt for step 03.
#
# conjure: DSSE envelope signature verification against GCB attestor keys.
#   Cryptographically proves provenance was signed by Google's build platform.
#   No slsa-verifier, no Python — jq + base64 + openssl only.
# bind: digest-pin comparison against upstream reference
# graft: GRAFTED stamp (no verification)

set -euo pipefail
echo "=== Mode-aware verification (${_RBGV_VESSEL_MODE}) ==="

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

MEDIA_TYPE=$(printf '%s' "${MANIFEST}" | jq -r '.mediaType // ""')
echo "Manifest media type: ${MEDIA_TYPE}"

IS_INDEX="false"
case "${MEDIA_TYPE}" in
  *manifest.list*|*image.index*) IS_INDEX="true" ;;
esac

if [ "${IS_INDEX}" = "true" ]; then
  echo "Multi-platform manifest detected"
  printf '%s' "${MANIFEST}" | jq -r '
    [.manifests[]
     | select((.platform.os != "unknown") or (.platform.architecture != "unknown"))
     | .platform.os + "/" + .platform.architecture
       + (if .platform.variant then "/" + .platform.variant else "" end)
    ] | join(",")
  ' > /workspace/vouch_platforms.txt
  test -s /workspace/vouch_platforms.txt || { echo "FATAL: No platforms found" >&2; exit 1; }
else
  echo "Single manifest detected"
  CONFIG_DIGEST=$(printf '%s' "${MANIFEST}" | jq -r '.config.digest')
  test -n "${CONFIG_DIGEST}" || { echo "FATAL: No config digest" >&2; exit 1; }
  CONFIG=$(curl -sf -H "Authorization: Bearer ${TOKEN}" "${REGISTRY_BASE}/blobs/${CONFIG_DIGEST}") \
    || { echo "FATAL: Failed to fetch config blob" >&2; exit 1; }
  printf '%s' "${CONFIG}" | jq -r '
    .os + "/" + .architecture
    + (if .variant then "/" + .variant else "" end)
  ' > /workspace/vouch_platforms.txt
fi
echo "Platforms for vouch: $(cat /workspace/vouch_platforms.txt)"

case "${_RBGV_VESSEL_MODE}" in

  conjure)
    BUILDER_ID="https://cloudbuild.googleapis.com/GoogleHostedWorker"
    EXPECTED_KEYID="projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1"
    VERIFY_METHOD="dsse-envelope"
    echo "Using DSSE envelope signature verification"

    # Extract per-platform digests
    if [ "${IS_INDEX}" = "true" ]; then
      printf '%s' "${MANIFEST}" | jq -r '
        .manifests[]
        | select((.platform.os != "unknown") or (.platform.architecture != "unknown"))
        | [.digest, .platform.architecture, (.platform.variant // "")] | join(" ")
      ' > /workspace/platform_entries.txt
    else
      # Single-platform: get architecture from config, look up alias tag digest
      ARCH=$(printf '%s' "${CONFIG}" | jq -r '.architecture')
      VARIANT=$(printf '%s' "${CONFIG}" | jq -r '.variant // ""')
      PLAT_SUFFIX="${ARCH}${VARIANT}"
      ALIAS_TAG="${_RBGV_CONSECRATION}${_RBGV_ARK_SUFFIX_IMAGE}-${PLAT_SUFFIX}"
      ALIAS_HEADERS=$(curl -sI \
        -H "Authorization: Bearer ${TOKEN}" \
        -H "Accept: ${ACCEPT}" \
        "${REGISTRY_BASE}/manifests/${ALIAS_TAG}") \
        || { echo "FATAL: HEAD request for alias tag ${ALIAS_TAG} failed" >&2; exit 1; }
      ALIAS_DIGEST=$(printf '%s' "${ALIAS_HEADERS}" | grep -i "docker-content-digest" | sed 's/^[^:]*: *//' | tr -d '\r\n')
      test -n "${ALIAS_DIGEST}" || { echo "FATAL: No digest for alias tag ${ALIAS_TAG}" >&2; exit 1; }
      echo "${ALIAS_DIGEST} ${ARCH} ${VARIANT}" > /workspace/platform_entries.txt
    fi

    DIGEST_COUNT=$(wc -l < /workspace/platform_entries.txt | tr -d ' ')
    echo "Found ${DIGEST_COUNT} platform entries"
    test "${DIGEST_COUNT}" -gt 0 || { echo "FATAL: no platform entries" >&2; exit 1; }

    while read -r DIGEST ARCH VARIANT; do
      PLAT_SUFFIX="${ARCH}${VARIANT}"
      FULL_REF="${FULL_IMAGE}@${DIGEST}"
      echo "Verifying ${PLAT_SUFFIX} (${DIGEST})..."

      # Fetch provenance from Container Analysis via gcloud
      echo "  Fetching provenance..."
      gcloud artifacts docker images describe "${FULL_REF}" \
        --format json --show-provenance \
        > "/workspace/provenance-${PLAT_SUFFIX}.json"

      # Extract v1.0 DSSE envelope (note name contains "intoto_slsa_v1")
      echo "  Extracting v1.0 DSSE envelope..."
      jq '
        .provenance_summary.provenance[]
        | select(.noteName | test("intoto_slsa_v1"))
        | .envelope
      ' "/workspace/provenance-${PLAT_SUFFIX}.json" \
        > "/workspace/envelope-${PLAT_SUFFIX}.json"
      test -s "/workspace/envelope-${PLAT_SUFFIX}.json" \
        || { echo "FATAL: No v1.0 DSSE envelope for ${PLAT_SUFFIX}" >&2; exit 1; }

      # Verify keyid matches expected attestor
      ACTUAL_KEYID=$(jq -r '.signatures[0].keyid' "/workspace/envelope-${PLAT_SUFFIX}.json")
      test "${ACTUAL_KEYID}" = "${EXPECTED_KEYID}" \
        || { echo "FATAL: Unexpected keyid: ${ACTUAL_KEYID} (expected ${EXPECTED_KEYID})" >&2; exit 1; }

      # Decode payload and signature from base64
      echo "  Decoding payload and signature..."
      jq -r '.payload' "/workspace/envelope-${PLAT_SUFFIX}.json" \
        | base64 -d > "/workspace/payload-${PLAT_SUFFIX}.raw"
      jq -r '.signatures[0].sig' "/workspace/envelope-${PLAT_SUFFIX}.json" \
        | base64 -d > "/workspace/sig-${PLAT_SUFFIX}.bin"

      # Reconstruct PAE (Pre-Authentication Encoding)
      # Format: "DSSEv1" SP LEN(payloadType) SP payloadType SP LEN(payload) SP payload
      echo "  Reconstructing PAE..."
      PAYLOAD_TYPE="application/vnd.in-toto+json"
      PAYLOAD_LEN=$(wc -c < "/workspace/payload-${PLAT_SUFFIX}.raw" | tr -d ' ')
      printf "DSSEv1 %d %s %d " "${#PAYLOAD_TYPE}" "${PAYLOAD_TYPE}" "${PAYLOAD_LEN}" \
        > "/workspace/pae-${PLAT_SUFFIX}.bin"
      cat "/workspace/payload-${PLAT_SUFFIX}.raw" >> "/workspace/pae-${PLAT_SUFFIX}.bin"

      # Verify ECDSA-P256-SHA256 signature
      echo "  Verifying DSSE signature..."
      openssl dgst -sha256 \
        -verify /workspace/keys/google-hosted-worker.pub \
        -signature "/workspace/sig-${PLAT_SUFFIX}.bin" \
        "/workspace/pae-${PLAT_SUFFIX}.bin" \
        || { echo "FATAL: DSSE signature verification FAILED for ${PLAT_SUFFIX}" >&2; exit 1; }

      # Provenance is now cryptographically trusted — validate fields
      echo "  Checking provenance fields..."
      ACTUAL_BUILDER=$(jq -r '.predicate.runDetails.builder.id' "/workspace/payload-${PLAT_SUFFIX}.raw")
      test "${ACTUAL_BUILDER}" = "${BUILDER_ID}" \
        || { echo "FATAL: builder.id mismatch: expected ${BUILDER_ID}, got ${ACTUAL_BUILDER}" >&2; exit 1; }

      ACTUAL_BUILD_TYPE=$(jq -r '.predicate.buildDefinition.buildType' "/workspace/payload-${PLAT_SUFFIX}.raw")
      SUBJECT_DIGEST=$(jq -r '.subject[0].digest.sha256' "/workspace/payload-${PLAT_SUFFIX}.raw")

      jq -n \
        --arg verifier "dsse-envelope-v1" \
        --arg builder_id "${ACTUAL_BUILDER}" \
        --arg build_type "${ACTUAL_BUILD_TYPE}" \
        --arg subject_digest "${SUBJECT_DIGEST}" \
        --arg keyid "${ACTUAL_KEYID}" \
        '{verifier: $verifier, builder_id: $builder_id, build_type: $build_type,
          subject_digest: $subject_digest, keyid: $keyid, verdict: "pass"}' \
        > "/workspace/verify-${PLAT_SUFFIX}.json"

      echo "  ${PLAT_SUFFIX}: DSSE verified OK"
    done < /workspace/platform_entries.txt

    echo "All ${DIGEST_COUNT} platforms verified via DSSE"

    # Compose vouch summary from individual verify files
    jq -n \
      --arg consecration "${_RBGV_CONSECRATION}" \
      --arg vessel "${_RBGV_VESSEL}" \
      --arg verify_method "${VERIFY_METHOD}" \
      '{consecration: $consecration, vessel: $vessel, vessel_mode: "conjure",
        verify_method: $verify_method, platforms: []}' \
      > /workspace/vouch_summary.json

    for f in /workspace/verify-*.json; do
      PLAT=$(basename "${f}" .json)
      PLAT="${PLAT#verify-}"
      jq --arg platform "${PLAT}" --slurpfile v "${f}" \
        '.platforms += [($v[0] + {platform: $platform})]' \
        /workspace/vouch_summary.json > /workspace/vouch_summary_tmp.json
      mv /workspace/vouch_summary_tmp.json /workspace/vouch_summary.json
    done
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
    jq -n \
      --arg consecration "${_RBGV_CONSECRATION}" \
      --arg vessel "${_RBGV_VESSEL}" \
      --arg bind_source "${_RBGV_BIND_SOURCE}" \
      --arg actual_digest "${ACTUAL_DIGEST}" \
      --arg pinned_digest "${PINNED_DIGEST}" \
      --arg verdict "${VERDICT}" \
      '{consecration: $consecration, vessel: $vessel, vessel_mode: "bind",
        verification: {method: "digest-pin", bind_source: $bind_source,
          actual_digest: $actual_digest, pinned_digest: $pinned_digest,
          verdict: $verdict}}' \
      > /workspace/vouch_summary.json
    echo "Vouch summary composed"
    ;;

  graft)
    echo "Graft mode — stamping GRAFTED"
    jq -n \
      --arg consecration "${_RBGV_CONSECRATION}" \
      --arg vessel "${_RBGV_VESSEL}" \
      --arg graft_source "${_RBGV_GRAFT_SOURCE}" \
      '{consecration: $consecration, vessel: $vessel, vessel_mode: "graft",
        verification: {method: "none", graft_source: $graft_source,
          verdict: "GRAFTED"}}' \
      > /workspace/vouch_summary.json
    echo "Vouch summary composed"
    ;;

  *) echo "FATAL: Unknown vessel mode: ${_RBGV_VESSEL_MODE}" >&2; exit 1 ;;
esac
