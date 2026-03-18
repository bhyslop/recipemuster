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

# Write direct provenance verifier for builds.create path (no git source)
cat > /workspace/direct_verify.py << 'PYEOF'
import json, sys
prov_file, out_file, expected_builder = sys.argv[1], sys.argv[2], sys.argv[3]
d = json.load(open(prov_file))
provs = d.get('provenance_summary', {}).get('provenance', [])
assert provs, 'No provenance found'
v1 = next((p['build']['inTotoSlsaProvenanceV1'] for p in provs if 'inTotoSlsaProvenanceV1' in p.get('build', {})), None)
assert v1, 'No v1 SLSA provenance'
pred = v1['predicate']
bid = pred.get('runDetails', {}).get('builder', {}).get('id', '')
bt = pred.get('buildDefinition', {}).get('buildType', '')
deps = pred.get('buildDefinition', {}).get('resolvedDependencies', [])
print(f'  builder.id: {bid}\n  buildType: {bt}\n  deps: {len(deps)}')
assert bid == expected_builder, f'builder.id mismatch: {bid}'
assert deps, 'No resolved dependencies'
json.dump({'verifier': 'direct-provenance-check', 'builder_id': bid, 'build_type': bt, 'resolved_dependencies': len(deps), 'verdict': 'pass'}, open(out_file, 'w'), indent=2)
print('  Direct provenance verification passed')
PYEOF
echo "direct_verify.py written"
