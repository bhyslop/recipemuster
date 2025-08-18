#!/bin/bash
set -euo pipefail

MY_TEMP="../../tmp-study-sgbs"
rm -rf "${MY_TEMP}"
mkdir -p "${MY_TEMP}"

# Accept only the OAuth token
z_token="${1:?Token required}"

# Hardcode GCP project/region and build upload URL
z_project="brm-recipemuster-proj"
z_region="us-central1"
z_url="https://cloudbuild.googleapis.com/upload/v1/projects/${z_project}/locations/${z_region}/builds"

z_boundary="__test_$$_${RANDOM}"

z_url_upload="https://cloudbuild.googleapis.com/upload/v1/projects/${z_project}/builds"
z_url_query="https://cloudbuild.googleapis.com/v1/projects/${z_project}/locations/${z_region}/builds?uploadType=multipart"

echo "SGBS: url (current): ${z_url}"
echo "SGBS: url (upload endpoint): ${z_url_upload}"
echo "SGBS: url (query uploadType): ${z_url_query}"


echo "SGBS: temp dir: ${MY_TEMP}"
echo "SGBS: url: ${z_url}"
echo "SGBS: boundary: ${z_boundary}"

echo "SGBS: 1) build metadata (cloudbuild substitutions)"
cat > "${MY_TEMP}/build.json" <<EOF
{"options": {"substitutionOption": "ALLOW_LOOSE"}}
EOF
echo "SGBS: build.json:"
cat "${MY_TEMP}/build.json"

# Create a tiny cloudbuild.yaml in the temp dir so tar doesn't depend on CWD
echo "SGBS: 2) make a tiny tar.gz with cloudbuild.yaml at root"
cat > "${MY_TEMP}/cloudbuild.yaml" <<'EOF'
steps:
- name: gcr.io/cloud-builders/busybox
  args: ["echo","study upload OK"]
EOF

tar -C "${MY_TEMP}" -czf "${MY_TEMP}/src.tgz" cloudbuild.yaml
ls -l "${MY_TEMP}/src.tgz"

echo "SGBS: 2b) tar contents of src.tgz"
tar -tzf "${MY_TEMP}/src.tgz" | sed 's/^/  - /'

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


echo "SGBS: body length: $(wc -c < "${MY_TEMP}/body.bin")"
echo "SGBS: body sha256: $(openssl dgst -sha256 "${MY_TEMP}/body.bin" | awk '{print $2}')"
echo "SGBS: body head (first 200 bytes)"; head -c 200 "${MY_TEMP}/body.bin" | od -An -tx1
echo
echo "SGBS: boundary probe"; (grep -a -n -- "--${z_boundary}" "${MY_TEMP}/body.bin" || true) | sed 's/^/  ln /'

echo "SGBS: Checking for CRLF vs LF:"
file "${MY_TEMP}/body.bin"

echo "SGBS: files:"
ls -l "${MY_TEMP}"
echo "SGBS: body length: $(wc -c < "${MY_TEMP}/body.bin")"

echo "SGBS: 4) POST (mode: header-only multipart to ${z_url})"
: > "${MY_TEMP}/resp.headers"
: > "${MY_TEMP}/resp.body"
: > "${MY_TEMP}/resp.code"

curl -v \
  -X POST \
  -H "Authorization: Bearer ${z_token}" \
  -H "Content-Type: multipart/related; boundary=${z_boundary}" \
  -H "Accept: application/json" \
  --data-binary @"${MY_TEMP}/body.bin" \
  -D "${MY_TEMP}/resp.headers" \
  -o "${MY_TEMP}/resp.body" \
  -w "%{http_code}" \
  "${z_url}" > "${MY_TEMP}/resp.code" 2>&1 || echo "SGBS: curl failed with status $?"

echo "SGBS: HTTP code: $(<"${MY_TEMP}/resp.code")"
echo "SGBS: response headers:"; sed 's/^/  /' "${MY_TEMP}/resp.headers" | head -n 50
echo "SGBS: response body head:"; head -c 500 "${MY_TEMP}/resp.body"; echo

# After the curl, if code is 200, show the logUrl for convenience
if test "$(tr -d '\r\n' < "${MY_TEMP}/resp.code")" = "200"; then
  echo "SGBS: build accepted."
  # Quick-and-dirty pull of logUrl (works even without jq)
  z_log_url=$(grep -ao '"logUrl"[^"]*"[^"]*"' "${MY_TEMP}/resp.body" | head -n1 | sed 's/.*"logUrl":"\([^"]*\)".*/\1/')
  test -n "${z_log_url}" && echo "SGBS: logUrl: ${z_log_url}"
fi

echo "SGBS: End."

