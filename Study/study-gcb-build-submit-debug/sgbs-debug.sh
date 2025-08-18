BOUND="__test_$$_$(date +%s)"
URL="https://cloudbuild.googleapis.com/upload/v1/projects/PROJECT/locations/REGION/builds?uploadType=multipart&alt=json"

echo "SGBS: 1) build metadata (your file ZRBF_BUILD_CONFIG_FILE)"
cat > /tmp/build.json <<'JSON'
{
  "substitutions": { "_RBGY_TAG": "dev" }
}
JSON

echo "SGBS: 2) make a tiny tar.gz with cloudbuild.yaml at root"
mkdir -p /tmp/wk && printf 'steps: []\n' > /tmp/wk/cloudbuild.yaml
tar -C /tmp/wk -czf /tmp/src.tgz .

echo "SGBS: 3) construct the multipart"
BODY=/tmp/body.bin
: > "$BODY"
{
  printf -- "--%s\r\n" "$BOUND"
  printf "Content-Type: application/json; charset=UTF-8\r\n\r\n"
  cat /tmp/build.json
  printf "\r\n--%s\r\n" "$BOUND"
  printf "Content-Type: application/gzip\r\n\r\n"
  cat /tmp/src.tgz
  printf "\r\n--%s--\r\n" "$BOUND"
} >> "$BODY"

echo "SGBS: 4) POST"
curl -v -X POST "$URL" \
  -H "Authorization: Bearer TOKEN" \
  -H "Accept: application/json" \
  -H "Content-Type: multipart/related; boundary=${BOUND}" \
  -H "X-Goog-Upload-Protocol: multipart" \
  -H "X-Goog-Upload-File-Name: source.tar.gz" \
  --data-binary @"$BODY"

echo "SGBS: End."
