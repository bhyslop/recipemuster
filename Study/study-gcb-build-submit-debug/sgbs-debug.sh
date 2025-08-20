#!/bin/bash
# Study: Cloud Build submit via GCS + JSON (single path)
# Usage:
#   ./sgbs-debug.sh "<OAUTH_TOKEN>" [PROJECT_ID] [REGION] [TAR_PATH]
# If TAR_PATH is omitted, a tiny cloudbuild.yaml tarball is created for you.
set -euo pipefail

echo  "---- Inputs -----------------------------------------------------------------"
z_token="${1:?Token required (OAuth2 access token)}"
z_project="${2:-brm-recipemuster-proj}"
z_region="${3:-us-central1}"
z_tar_in="${4:-}"

echo  " ---- Scratch ----------------------------------------------------------------"
MY_TEMP="../../../tmp-study-sgbs"
rm -rf "${MY_TEMP}"
mkdir -p "${MY_TEMP}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"

echo  "Hardcode bucket from apparatus (created during admin init)"
z_bucket="brm-recipemuster-proj-artifacts"
z_object="source/sgbs-${STAMP}.tgz"

echo  "---- Bucket probe -------------------------------------------------"
curl -sS -H "Authorization: Bearer ${z_token}" \
  "https://storage.googleapis.com/storage/v1/b?project=${z_project}"

echo  "---- Bucket existence check -------------------------------------------------"
echo "SGBS: Bucket ${z_bucket} check skipped due to lack of permissions."

echo  "---- Endpoints --------------------------------------------------------------"
# FIX: preserve slashes; do not transform them (previous sed replaced '/' with '%')
z_gcs_upload="https://storage.googleapis.com/upload/storage/v1/b/${z_bucket}/o?uploadType=media&name=${z_object}"
z_cb_create="https://cloudbuild.googleapis.com/v1/projects/${z_project}/locations/${z_region}/builds"

echo "SGBS: project/region: ${z_project}/${z_region}"
echo "SGBS: bucket/object:  gs://${z_bucket}/${z_object}"
echo "SGBS: GCS upload:     ${z_gcs_upload}"
echo "SGBS: CB create:      ${z_cb_create}"

echo  "---- Prepare a tarball (or use the one provided) ----------------------------"
if test -z "${z_tar_in}"; then
  echo "SGBS: No TAR_PATH provided -> making a tiny tar with cloudbuild.yaml at root"
  cat > "${MY_TEMP}/cloudbuild.yaml" <<'YAML'
steps:
- name: gcr.io/cloud-builders/busybox
  args: ["echo","study upload OK"]
YAML
  tar -C "${MY_TEMP}" -czf "${MY_TEMP}/src.tgz" cloudbuild.yaml
  z_tar="${MY_TEMP}/src.tgz"
else
  echo "SGBS: Using provided TAR_PATH: ${z_tar_in}"
  z_tar="${z_tar_in}"
fi

ls -l "${z_tar}"
echo "SGBS: tar sha256: $(openssl dgst -sha256 "${z_tar}" | awk '{print $2}')"

echo  "---- 1) Upload tar.gz to GCS (objects.insert: uploadType=media) ------------"
echo "SGBS: === GCS upload ==="
: > "${MY_TEMP}/gcs.headers"
: > "${MY_TEMP}/gcs.body"
: > "${MY_TEMP}/gcs.code"

curl -sS -X POST \
  -H "Authorization: Bearer ${z_token}" \
  -H "Content-Type: application/gzip" \
  --data-binary @"${z_tar}" \
  -D "${MY_TEMP}/gcs.headers" \
  -o "${MY_TEMP}/gcs.body" \
  -w "%{http_code}" \
  "${z_gcs_upload}" > "${MY_TEMP}/gcs.code" || echo "SGBS: curl (GCS) exited nonzero"

echo "SGBS: GCS HTTP code: $(<"${MY_TEMP}/gcs.code")"
echo "SGBS: GCS response headers (first 20):"; sed 's/^/  /' "${MY_TEMP}/gcs.headers" | head -n 20
echo "SGBS: GCS response body head:"; head -c 400 "${MY_TEMP}/gcs.body"; echo

gcs_code="$(tr -d '\r\n' < "${MY_TEMP}/gcs.code")"
if test "${gcs_code}" != "200" && test "${gcs_code}" != "201"; then
  echo "SGBS: GCS upload failed (expect 200/201). Stop." >&2
  exit 1
fi

# FIX: adopt the exact stored object name that GCS reports (prevents NOT_FOUND later)
stored_name=""
if command -v jq >/dev/null 2>&1; then
  stored_name="$(jq -r '.name // empty' "${MY_TEMP}/gcs.body")"
else
  stored_name="$(grep -ao '"name" *: *"[^"]*"' "${MY_TEMP}/gcs.body" | head -n1 | sed 's/.*"name"[^\"]*"\([^\"]*\)".*/\1/')"
fi
test -n "${stored_name}" || { echo "SGBS: Could not parse stored object name from GCS response"; exit 1; }

if test "${stored_name}" != "${z_object}"; then
  echo "SGBS: WARNING: requested object '${z_object}' but GCS stored as '${stored_name}' — using stored name."
  z_object="${stored_name}"
fi

echo  "---- 2) Build JSON for Cloud Build create -----------------------------------"
echo "SGBS: === Build JSON (storageSource -> cloudbuild.yaml at root) ==="
cat > "${MY_TEMP}/build.json" <<JSON
{
  "source": {
    "storageSource": {
      "bucket": "${z_bucket}",
      "object": "${z_object}"
    }
  },
  "options": { "substitutionOption": "ALLOW_LOOSE" },
  "substitutions": {}
}
JSON

cat "${MY_TEMP}/build.json" | sed 's/^/  /'

echo  "---- 3) Call builds.create (JSON only) --------------------------------------"
echo "SGBS: === Cloud Build create ==="
: > "${MY_TEMP}/cb.headers"
: > "${MY_TEMP}/cb.body"
: > "${MY_TEMP}/cb.code"

curl -sS -X POST \
  -H "Authorization: Bearer ${z_token}" \
  -H "Accept: application/json" \
  -H "Content-Type: application/json; charset=UTF-8" \
  --data-binary @"${MY_TEMP}/build.json" \
  -D "${MY_TEMP}/cb.headers" \
  -o "${MY_TEMP}/cb.body" \
  -w "%{http_code}" \
  "${z_cb_create}" > "${MY_TEMP}/cb.code" || echo "SGBS: curl (CB) exited nonzero"

cb_code="$(tr -d '\r\n' < "${MY_TEMP}/cb.code")"
echo "SGBS: CB HTTP code: ${cb_code}"
echo "SGBS: CB response headers (first 30):"; sed 's/^/  /' "${MY_TEMP}/cb.headers" | head -n 30
echo "SGBS: CB response body head:"; head -c 600 "${MY_TEMP}/cb.body"; echo

if test "${cb_code}" != "200"; then
  echo "SGBS: builds.create did not return 200. Stop." >&2
  exit 1
fi

echo  "---- 4) Try to extract Operation + build id ---------------------------------"
echo "SGBS: === Parse operation ==="
build_id=""
op_name=""
log_url=""

if command -v jq >/dev/null 2>&1; then
  op_name="$(jq -r '.name // empty' "${MY_TEMP}/cb.body")"
  build_id="$(jq -r '.metadata.build.id // empty' "${MY_TEMP}/cb.body")"
  log_url="$(jq -r '.metadata.build.logUrl // empty' "${MY_TEMP}/cb.body")"
else
  op_name="$(grep  -ao '"name" *: *"[^"]*"' "${MY_TEMP}/cb.body"   | head -n1 | sed 's/.*"name"[^\"]*"\([^\"]*\)".*/\1/')"
  build_id="$(grep -ao '"id" *: *"[^"]*"' "${MY_TEMP}/cb.body"     | head -n1 | sed 's/.*"id"[^\"]*"\([^\"]*\)".*/\1/')"
  log_url="$(grep  -ao '"logUrl" *: *"[^"]*"' "${MY_TEMP}/cb.body" | head -n1 | sed 's/.*"logUrl"[^\"]*"\([^\"]*\)".*/\1/')"
fi

echo "SGBS: operation.name: ${op_name:-<empty>}"
echo "SGBS: metadata.build.id: ${build_id:-<empty>}"
echo "SGBS: metadata.build.logUrl: ${log_url:-<empty>}"

if test -n "${build_id}"; then
  console_url="https://console.cloud.google.com/cloud-build/builds/${build_id}?project=${z_project}"
  echo "SGBS: Console: ${console_url}"
else
  echo "SGBS: (No build id yet; inspect ${MY_TEMP}/cb.body for details.)"
fi

echo "SGBS: End."

