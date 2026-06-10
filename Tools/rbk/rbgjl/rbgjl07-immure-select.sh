#!/bin/bash
# RBGJL Step 07: Select podvm disk leaves from a quay family index (the immure select)
# Builder: gcr.io/cloud-builders/docker (Google-hosted, always pullable; Debian-based
#          — carries curl, and jq is apt-installed below if absent). This step does
#          the JSON-bearing work the gcrane:debug capture step (rbgjl08, busybox: no
#          jq, no curl) cannot: it anon-reads the PUBLIC quay family index over the
#          registry-v2 API, selects the curated {disktype × arch} leaves by their
#          index child DESCRIPTOR (platform.architecture + annotations.disktype, never
#          the layer filename — unreliable per memo-20260608 §3.4), authors the
#          recorded-grade provenance envelope, and stages a selection list for the
#          capture + residency + vouch steps that follow. Mirrors underpin's split
#          (rbgjl04 fetch/verify on Debian -> rbgjl05 wrap on gcrane): the JSON tool
#          (jq) and the registry tool (gcrane) live in disjoint builder images.
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT, _RBGL_LODE_STAMP,
#                _RBGL_TAG_SPRUE, _RBGL_TRUST_GRADE, _RBGL_VOUCH_SCHEMA,
#                _RBGL_ACQUIRED_BY, _RBGL_PODVM_BRAND, _RBGL_PODVM_FAMILY,
#                _RBGL_PODVM_VERSION, _RBGL_PODVM_SELECTION
#
# Note: this script runs inside a Cloud Build container, not under BCG module
# discipline (CBG governs).
#
# podvm is recorded-at-acquisition: quay rotates these images out within days and
# publishes no durable checksum, so RB attests only the leaf digest observed at
# capture. There is no checksum to verify (unlike wsl); the integrity guard is the
# downstream blob-residency HEAD (rbgjl09) plus gcrane cp's digest-faithful copy.
#
# Selection list staged at /workspace/immure_selection.txt — one row per selected
# leaf, the contract for rbgjl08 (capture) and rbgjl09 (residency):
#   <member_tag>|<leaf_manifest_digest>|<layer_blob_digest>|<layer_blob_size>
# The envelope (/workspace/lode_<stamp>_vouch.json) and the stamp roster
# (/workspace/lode_stamps.txt) are the contract for rbgjl02 (vouch push).

set -euo pipefail
echo "=== Select podvm disk leaves from quay family index ==="

STAMP="${_RBGL_LODE_STAMP}"
FAMILY="${_RBGL_PODVM_FAMILY}"
VERSION="${_RBGL_PODVM_VERSION}"
SELECTION="${_RBGL_PODVM_SELECTION}"
test -n "${STAMP}"     || { echo "FATAL: _RBGL_LODE_STAMP missing"     >&2; exit 1; }
test -n "${FAMILY}"    || { echo "FATAL: _RBGL_PODVM_FAMILY missing"   >&2; exit 1; }
test -n "${VERSION}"   || { echo "FATAL: _RBGL_PODVM_VERSION missing"  >&2; exit 1; }
test -n "${SELECTION}" || { echo "FATAL: _RBGL_PODVM_SELECTION missing">&2; exit 1; }

# jq is the JSON contract of this step; ensure it on the Debian builder (apt path
# mirrors rbgjs-gpg-verify-sums). A missing jq fails loud rather than skipping.
if ! command -v jq >/dev/null 2>&1; then
  echo "--- jq absent — apt-get install ---"
  if ! { apt-get update >/dev/null 2>&1 && apt-get install -y jq >/dev/null 2>&1; }; then
    echo "FATAL: jq not present and apt-get install failed — cannot parse the index" >&2
    exit 1
  fi
fi

# Decompose the family into the registry-v2 host + repository. podvm families are
# always quay.io/podman/<name>, but derive generically so a host change is data.
REGISTRY="${FAMILY%%/*}"        # quay.io
REPO="${FAMILY#*/}"             # podman/machine-os-wsl
test "${REGISTRY}" != "${FAMILY}" || { echo "FATAL: family '${FAMILY}' has no registry/repo split" >&2; exit 1; }
echo "Registry: ${REGISTRY}  Repo: ${REPO}  Version: ${VERSION}"

PKG="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}"
echo "Lode package: ${PKG}"

# --- Anon registry-v2 token for the PUBLIC quay repo (pull scope) ---
# quay.io issues an anonymous bearer for public pulls at /v2/auth. The response
# carries the token under .token (quay) or .access_token (some registries) — accept
# either. This is a read of upstream's public index, not a GAR push; gcrane handles
# the GAR auth ambiently in the capture step.
echo "--- Acquiring anon pull token for ${REPO} ---"
AUTH_URL="https://${REGISTRY}/v2/auth?service=${REGISTRY}&scope=repository:${REPO}:pull"
ANON_JSON=$(curl -sf "${AUTH_URL}") \
  || { echo "FATAL: failed to acquire anon pull token from ${AUTH_URL}" >&2; exit 1; }
PULL_TOKEN=$(printf '%s' "${ANON_JSON}" | jq -r '.token // .access_token // empty')
test -n "${PULL_TOKEN}" || { echo "FATAL: no token in anon auth response" >&2; exit 1; }

# Helper: GET a manifest by reference (tag or digest) with the pull token. Accepts
# both OCI and docker manifest/index media types so either family layout resolves.
ACCEPT='application/vnd.oci.image.index.v1+json,application/vnd.docker.distribution.manifest.list.v2+json,application/vnd.oci.image.manifest.v1+json,application/vnd.docker.distribution.manifest.v2+json'
fetch_manifest() {  # $1 = reference (tag or sha256:...)  $2 = output file
  curl -sf -H "Authorization: Bearer ${PULL_TOKEN}" -H "Accept: ${ACCEPT}" \
    "https://${REGISTRY}/v2/${REPO}/manifests/${1}" -o "${2}" \
    || { echo "FATAL: failed to fetch manifest ${1}" >&2; exit 1; }
}

# --- Fetch the family index at the pinned version ---
echo "--- Fetching index ${FAMILY}:${VERSION} ---"
INDEX="/workspace/immure_index_${STAMP}.json"
fetch_manifest "${VERSION}" "${INDEX}"

# The top-level reference must be a multi-arch index — podvm families publish one
# index per version since 5.4 (memo-20260608 §3.2/§3.3). A single image here means
# the version/family pairing is wrong; fail loud rather than capture a non-leaf.
INDEX_TYPE=$(jq -r '.mediaType // empty' "${INDEX}")
case "${INDEX_TYPE}" in
  *image.index*|*manifest.list*) : ;;
  *) echo "FATAL: ${FAMILY}:${VERSION} is not an index (mediaType: ${INDEX_TYPE:-absent})" >&2; exit 1 ;;
esac

# --- Select the curated leaves and build the envelope members[] ---
ACQUIRED_AT=$(date -u +%Y-%m-%dT%H:%M:%SZ)
: > /workspace/immure_selection.txt
MEMBERS=''
MFIRST=true

# SELECTION is space-separated disktype:arch rows. For each, find the index child
# descriptor matching platform.architecture + annotations.disktype, then fetch that
# leaf manifest to read its single layer's digest + size (the residency guard
# compares the GAR blob's Content-Length against that declared size).
for ROW in ${SELECTION}; do
  DISKTYPE="${ROW%%:*}"
  ARCH="${ROW##*:}"
  test -n "${DISKTYPE}" && test -n "${ARCH}" \
    || { echo "FATAL: malformed selection row '${ROW}' (want disktype:arch)" >&2; exit 1; }
  MEMBER_TAG="${_RBGL_TAG_SPRUE}${DISKTYPE}-${ARCH}"
  echo "--- Selecting ${DISKTYPE}/${ARCH} -> :${MEMBER_TAG} ---"

  # Match on the descriptor, never the layer filename. The disktype leaves carry
  # arch in the alt spelling (x86_64/aarch64) and disktype in annotations.
  LEAF_DIGEST=$(jq -r --arg a "${ARCH}" --arg d "${DISKTYPE}" '
    [ .manifests[]
      | select(.platform.architecture == $a and (.annotations.disktype // "") == $d)
      | .digest ] | first // empty
  ' "${INDEX}")
  test -n "${LEAF_DIGEST}" \
    || { echo "FATAL: no leaf for ${DISKTYPE}/${ARCH} in ${FAMILY}:${VERSION}" >&2; exit 1; }

  # The leaf is a single-platform OCI artifact: empty config + ONE zstd blob layer.
  # Read its layer digest + size for the residency guard and the envelope. Exactly
  # one layer is expected (memo-20260608 §3.4); more than one means the descriptor
  # matched a non-disk manifest — fail loud.
  LEAF="/workspace/immure_leaf_${DISKTYPE}_${ARCH}.json"
  fetch_manifest "${LEAF_DIGEST}" "${LEAF}"
  LAYER_COUNT=$(jq -r '.layers | length' "${LEAF}")
  test "${LAYER_COUNT}" = "1" \
    || { echo "FATAL: leaf ${DISKTYPE}/${ARCH} has ${LAYER_COUNT} layers (expected 1 disk blob)" >&2; exit 1; }
  BLOB_DIGEST=$(jq -r '.layers[0].digest' "${LEAF}")
  BLOB_SIZE=$(jq -r '.layers[0].size' "${LEAF}")
  test -n "${BLOB_DIGEST}" && test -n "${BLOB_SIZE}" \
    || { echo "FATAL: leaf ${DISKTYPE}/${ARCH} missing layer digest/size" >&2; exit 1; }
  echo "  leaf manifest: ${LEAF_DIGEST}"
  echo "  disk blob:     ${BLOB_DIGEST} (${BLOB_SIZE} bytes)"

  # Stage the selection row (capture + residency contract).
  printf '%s|%s|%s|%s\n' "${MEMBER_TAG}" "${LEAF_DIGEST}" "${BLOB_DIGEST}" "${BLOB_SIZE}" \
    >> /workspace/immure_selection.txt

  # Accumulate the envelope member. Recorded grade: the attestation IS the captured
  # leaf digest (trust-on-first-acquisition); verification "recorded" never implies
  # the bytes remain re-checkable against a vanished upstream. No jq for assembly —
  # values are controlled (member tag, family ref, hex digests); none carry a quote.
  if [ "${MFIRST}" = "true" ]; then MFIRST=false; else MEMBERS="${MEMBERS},"; fi
  MEMBERS="${MEMBERS}{"
  MEMBERS="${MEMBERS}\"name\":\"${MEMBER_TAG}\","
  MEMBERS="${MEMBERS}\"origin\":\"${FAMILY}:${VERSION}\","
  MEMBERS="${MEMBERS}\"digest\":\"${LEAF_DIGEST}\","
  MEMBERS="${MEMBERS}\"verification\":\"recorded\","
  MEMBERS="${MEMBERS}\"tags\":[\"${MEMBER_TAG}\"]"
  MEMBERS="${MEMBERS}}"
done

test -s /workspace/immure_selection.txt \
  || { echo "FATAL: selection produced no leaves — nothing to immure" >&2; exit 1; }

# --- Author the batch provenance envelope (identical content lands in :rbi_vouch
# and the host capture-file). kind == the brand; members[] is the cardinality axis
# (N for the podvm cohort). ---
ENVELOPE='{'
ENVELOPE="${ENVELOPE}\"schema\":\"${_RBGL_VOUCH_SCHEMA}\","
ENVELOPE="${ENVELOPE}\"kind\":\"${_RBGL_PODVM_BRAND}\","
ENVELOPE="${ENVELOPE}\"lode\":\"${STAMP}\","
ENVELOPE="${ENVELOPE}\"acquired_at\":\"${ACQUIRED_AT}\","
ENVELOPE="${ENVELOPE}\"acquired_by\":\"${_RBGL_ACQUIRED_BY}\","
ENVELOPE="${ENVELOPE}\"capture_build\":\"${BUILD_ID:-}\","
ENVELOPE="${ENVELOPE}\"trust_grade\":\"${_RBGL_TRUST_GRADE}\","
ENVELOPE="${ENVELOPE}\"signature\":null,"
ENVELOPE="${ENVELOPE}\"members\":[${MEMBERS}]}"

printf '%s' "${ENVELOPE}" > "/workspace/lode_${STAMP}_vouch.json"
: > /workspace/lode_stamps.txt
echo "${STAMP}" >> /workspace/lode_stamps.txt

# Host-facing result (the capture-file carries the same envelope). One slot —
# immure produces exactly one Lode (the cohort is one package).
RESULT="{\"slot_1\":{\"stamp\":\"${STAMP}\",\"vouch\":${ENVELOPE}}}"

echo "=== Writing selection results ==="
cat /workspace/immure_selection.txt

# Write to buildStepOutputs channel (host extracts the touchmark -> capture-file).
mkdir -p /builder/outputs
printf '%s' "${RESULT}" > /builder/outputs/output

echo "=== Immure select step complete ==="
