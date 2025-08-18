#!/bin/bash
# Minimal GCB multipart upload study (parameterized)
# Usage: ./sgbs-debug.sh PROJECT REGION TOKEN
set -euo pipefail

# Inputs
PROJECT="${1:-}"; test -n "${PROJECT}" || { echo "PROJECT required" >&2; exit 2; }
REGION="${2:-}";  test -n "${REGION}"  || { echo "REGION required"  >&2; exit 2; }
TOKEN="${3:-}";   test -n "${TOKEN}"   || { echo "TOKEN required"   >&2; exit 2; }

# Temp dir management (default ../tmp alongside this script)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MY_TEMP_DEFAULT="${SCRIPT_DIR}/../tmp"
MY_TEMP="${MY_TEMP:-${MY_TEMP_DEFAULT}}"
rm -rf "${MY_TEMP}" || true
mkdir -p "${MY_TEMP}"

BOUND="__test_$$_${EPOCHSECONDS:-$(date +%s)}"
URL="https://cloudbuild.googleapis.com/upload/v1/projects/${PROJECT}/locations/${REGION}/builds?uploadType=multipart&alt=json"

echo "SGBS: 1) build metadata (your file ZRBF_BUILD_CONFIG_FILE)"
BUILD_JSON="${MY_TEMP}/build.json"
: > "${BUILD_JSON}"
echo '{'                                 >> "${BUILD_JSON}"
echo '  "substitutions": { "_RBGY_TAG": "dev" }' >> "${BUILD_JSON}"
echo '}'                                 >> "${BUILD_JSON}"

echo "SGBS: 2) make a tiny tar.gz with cloudbuild.yaml at root"
WK="${MY_TEMP}/wk"
mkdir -p "${WK}"
printf 'steps: []\n' > "${WK}/cloudbuild.yaml"
SRC_TGZ="${MY_TEMP}/src.tgz"
tar -C "${WK}" -czf "${SRC_TGZ}" .

echo "SGBS: 3) construct the multipart"
BODY="${MY_TEMP}/body.bin"
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

echo "SGBS: 4) POST"
curl -v -X POST "${URL}" \
  -H "Authorization: Bearer ${TOKEN}" \
  -H "Accept: application/json" \
  -H "Content-Type: multipart/related; boundary=${BOUND}" \
  -H "X-Goog-Upload-Protocol: multipart" \
  -H "X-Goog-Upload-File-Name: source.tar.gz" \
  --data-binary @"${BODY}"

echo "SGBS: End."

