#!/bin/sh
# RBGJV Step 01: Download and verify slsa-verifier binary (conjure only)
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_VESSEL_MODE, _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256

set -eu
echo "=== Download and verify slsa-verifier ==="

# Early-exit for non-conjure modes (slsa-verifier only needed for SLSA provenance verification)
if [ "${_RBGV_VESSEL_MODE}" != "conjure" ]; then
  echo "Vessel mode is ${_RBGV_VESSEL_MODE} — skipping slsa-verifier download"
  exit 0
fi

wget -q -O /workspace/slsa-verifier "${_RBGV_VERIFIER_URL}"
COMPUTED=$(sha256sum /workspace/slsa-verifier | cut -d ' ' -f1)
if [ "${COMPUTED}" != "${_RBGV_VERIFIER_SHA256}" ]; then
  echo "FATAL: checksum mismatch" >&2
  echo "  expected: ${_RBGV_VERIFIER_SHA256}" >&2
  echo "  computed: ${COMPUTED}" >&2
  exit 1
fi
chmod +x /workspace/slsa-verifier
echo "slsa-verifier verified"
