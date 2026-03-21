#!/bin/sh
# RBGJV Step 01: Prepare DSSE verification keys (conjure only)
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_VESSEL_MODE

set -eu
echo "=== Prepare verification keys ==="

# Early-exit for non-conjure modes (DSSE verification only needed for conjure)
if [ "${_RBGV_VESSEL_MODE}" != "conjure" ]; then
  echo "Vessel mode is ${_RBGV_VESSEL_MODE} — skipping key setup"
  exit 0
fi

# Write GCB attestor public keys to workspace
# Source: projects/verified-builder KMS (embedded — keys change rarely)
# Verification: DSSE envelope signatures on GCB-generated SLSA provenance
mkdir -p /workspace/keys

# v1.0 provenance key: google-hosted-worker (global, DSSE PAE)
# KMS path: projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1
cat > /workspace/keys/google-hosted-worker.pub << 'KEYEOF'
-----BEGIN PUBLIC KEY-----
MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEg9KII7kzr/30HBluf00y9WwtMFkE
qc3oCcFVH3QJ37IBLUv/MUApbnNHFfD75ayJ/a0F45xa+MLv5zoep+GxsA==
-----END PUBLIC KEY-----
KEYEOF

echo "Attestor public keys written to /workspace/keys/"
