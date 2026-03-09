#!/bin/sh
# RBGJV Step 01: Download and verify slsa-verifier binary
# Builder: alpine (via RBRG_ALPINE_IMAGE_REF)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_VERIFIER_URL, _RBGV_VERIFIER_SHA256

set -eu
echo "=== Download and verify slsa-verifier ==="
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
