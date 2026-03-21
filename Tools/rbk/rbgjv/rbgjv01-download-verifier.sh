#!/bin/sh
# RBGJV Step 01: Prepare verification tools and keys
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_VESSEL_MODE

set -eu
echo "=== Prepare verification tools ==="

# Install statically-linked jq to /workspace/bin/ (gcloud image lacks jq;
# alpine's jq is musl-linked and fails on glibc with "No such file or directory")
# Required for ALL modes — step 02 uses jq for platform discovery and vouch summary
mkdir -p /workspace/bin
wget -q -O /workspace/bin/jq "https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64"
chmod +x /workspace/bin/jq
/workspace/bin/jq --version
echo "jq installed to /workspace/bin/"

# Conjure-only: write GCB attestor public key for DSSE envelope verification
if [ "${_RBGV_VESSEL_MODE}" = "conjure" ]; then
  mkdir -p /workspace/keys
  # KMS: projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1
  cat > /workspace/keys/google-hosted-worker.pub << 'KEYEOF'
-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEg9KII7kzr/30HBluf00y9WwtMFkE
qc3oCcFVH3QJ37IBLUv/MUApbnNHFfD75ayJ/a0F45xa+MLv5zoep+GxsA==
-----END PUBLIC KEY-----
KEYEOF
  echo "Attestor public keys written to /workspace/keys/"
fi
