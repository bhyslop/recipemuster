#!/bin/bash
set -euo pipefail

# --- run from the study dir; keep it self-contained ---
SDIR="${BASH_SOURCE[0]%/*}"
cd "${SDIR}"

# --- single temp root; clear & recreate each run ---
MY_TEMP="../../../tmp-study-sgbs"
rm -rf "${MY_TEMP}" 2>/dev/null || true
mkdir -p "${MY_TEMP}"

# --- inputs from rbf_study / caller ---
PROJECT="${1:?project id required}"
REGION="${2:?region required}"
TOKEN="${3:?oauth token required}"

BOUND="__test_$$_$(date +%s)"
URL="https://cloudbuild.googleapis.com/upload/v1/projects/${PROJECT}/locations/${REGION}/builds?uploadType=multipart&alt=json"

# --- file paths (ALL under MY_TEMP) ---
BUILD_JSON="${MY_TEMP}/build.json"
SRC_TGZ="${MY_TEMP}/src.tgz"
BODY="${MY_TEMP}/body.bin"

echo "SGBS: temp dir: ${MY_TEMP}"
echo "SGBS: url: ${URL}"
echo "SGBS: boundary: ${BOUND}"

echo "SGBS: 1) build metadata (cloudbuild substitutions)"
printf '{ "substitutions": { "_RBGY_TAG": "dev" } }\n' > "${BUILD_JSON}"
test -s "${BUILD_JSON}" || { echo "empty ${BUILD_JSON}"; exit 1; }

echo "SGBS: 2) make a tiny tar.gz with cloudbuild.yaml at root"
WK="${MY_TEMP}/wk"
mkdir -p "${WK}"
printf 'steps: []\n' > "${WK}/cloudbuild.yaml"
tar -C "${WK}" -czf "${SRC_TGZ}" .
test -s "${SRC_TGZ}" || { echo "empty ${SRC_TGZ}"; exit 1; }

echo "SGBS: 3) construct the multipart"
: > "${BODY}"
{
  printf -- "--%s\r\n" "${BOUND}"
  printf "Content-Type: application/json; charset=UTF-8\r\n\r\n"
  cat    "${BUILD_JSON}"
  printf "\r\n--%s\r\n" "${BOUND}"
  printf "Content-Type: application/gzip\r\n\r\n"
  cat    "${SRC_TGZ}"
  printf "\r\n--%s--\r\n" "${BOUND}"
} >> "${BODY}"
test -s "${BODY}" || { echo "empty ${BODY}"; exit 1; }

# extra visibility
echo "SGBS: files:"
ls -l "${BUILD_JSON}" "${SRC_TGZ}" "${BODY}"

# optional: precompute Content-Length (not required, but helpful)
LEN="$(wc -c < "${BODY}")"
echo "SGBS: body length: ${LEN}"

echo "SGBS: 4) POST"
# IMPORTANT: quote TOKEN only in header (don’t echo it), and use the local BODY path
curl -v -X POST "${URL}" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: application/json" \
  -H "Content-Type: multipart/related; boundary=${BOUND}" \
  -H "X-Goog-Upload-Protocol: multipart" \
  -H "X-Goog-Upload-File-Name: source.tar.gz" \
  -H "Content-Length: ${LEN}" \
  --data-binary @"${BODY}"

echo "SGBS: End."

