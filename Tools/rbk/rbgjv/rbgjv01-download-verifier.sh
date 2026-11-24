#!/bin/sh
# Copyright 2026 Scale Invariant, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# RBGJV Step 01: Prepare verification keys
# Builder: alpine (from reliquary)
# Entrypoint: sh (not bash — alpine does not have bash)
# Substitutions: _RBGV_VESSEL_MODE

set -eu
echo "=== Prepare verification keys ==="

# Conjure-only: write GCB attestor public key for DSSE envelope verification
if [ "${_RBGV_VESSEL_MODE}" = "rbnve_conjure" ]; then
  mkdir -p /workspace/keys
  # KMS: projects/verified-builder/locations/global/keyRings/attestor/cryptoKeys/google-hosted-worker/cryptoKeyVersions/1
  {
    printf '%s\n' '-----BEGIN PUBLIC KEY-----'
    printf '%s\n' 'MFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAEg9KII7kzr/30HBluf00y9WwtMFkE'
    printf '%s\n' 'qc3oCcFVH3QJ37IBLUv/MUApbnNHFfD75ayJ/a0F45xa+MLv5zoep+GxsA=='
    printf '%s\n' '-----END PUBLIC KEY-----'
  } > /workspace/keys/google-hosted-worker.pub
  echo "Attestor public keys written to /workspace/keys/"
fi
