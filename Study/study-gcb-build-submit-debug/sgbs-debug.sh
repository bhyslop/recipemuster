#!/bin/bash
set -euo pipefail

MY_TEMP="../../tmp-study-sgbs"
rm -rf "${MY_TEMP}"
mkdir -p "${MY_TEMP}"

z_url="${1:?URL required}"
z_token="${2:?Token required}"
z_boundary="__test_$$_${RANDOM}"

echo "SGBS: temp dir: ${MY_TEMP}"
echo "SGBS: url: ${z_url}"
echo "SGBS: boundary: ${z_boundary}"

echo "SGBS: 1) build metadata (cloudbuild substitutions)"
cat > "${MY_TEMP}/build.json" <<EOF
{"options": {"substitutionOption": "ALLOW_LOOSE"}}
EOF
echo "SGBS: build.json:"
cat "${MY_TEMP}/build.json"

echo "SGBS: 2) make a tiny tar.gz with cloudbuild.yaml at root"
tar -C . -czf "${MY_TEMP}/src.tgz" cloudbuild.yaml
ls -l "${MY_TEMP}/src.tgz"

echo "SGBS: 3) construct the multipart"
{
  echo "--${z_boundary}"
  echo "Content-Type: application/json; charset=UTF-8"
  echo
  cat "${MY_TEMP}/build.json"
  echo
  echo "--${z_boundary}"
  echo "Content-Type: application/octet-stream"
  echo "Content-Disposition: form-data; name=\"file\"; filename=\"source.tar.gz\""
  echo
  cat "${MY_TEMP}/src.tgz"
  echo
  echo "--${z_boundary}--"
} > "${MY_TEMP}/body.bin"

echo "SGBS: files:"
ls -l "${MY_TEMP}"
echo "SGBS: body length: $(wc -c < "${MY_TEMP}/body.bin")"

echo "SGBS: 4) POST"
curl -v \
  -X POST \
  -H "Authorization: Bearer ${z_token}" \
  -H "Content-Type: multipart/related; boundary=${z_boundary}" \
  -H "X-Goog-Upload-Protocol: multipart" \
  -H "X-Goog-Upload-File-Name: source.tar.gz" \
  --data-binary @"${MY_TEMP}/body.bin" \
  "${z_url}" || echo "SGBS: curl failed with status $?"

echo "SGBS: End."

