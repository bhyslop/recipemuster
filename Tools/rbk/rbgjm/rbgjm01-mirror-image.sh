#!/busybox/sh
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
# RBGJM Step 01: Mirror image from upstream to GAR via gcrane
# Builder: gcrane (from reliquary — :debug variant carries /busybox/sh)
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_HALLMARKS_ROOT,
#                _RBGA_HALLMARK, _RBGA_BIND_SOURCE
#
# Registry-to-registry copy preserving multi-platform manifest lists.
# gcrane authenticates GAR ambiently via its google.Keychain (Mason SA,
# GCE metadata server) — no token fetch needed.
#
# Destination uses canonical AAK shape:
#   ${gar_host}/${gar_path}/${HALLMARKS_ROOT}/${hallmark}/image:${hallmark}
# matching rbgja02 (syft), rbgja04 (about), rbgjv03 (vouch).

set -eu

test -n "${_RBGA_GAR_HOST}"        || { echo "_RBGA_GAR_HOST missing"        >&2; exit 1; }
test -n "${_RBGA_GAR_PATH}"        || { echo "_RBGA_GAR_PATH missing"        >&2; exit 1; }
test -n "${_RBGA_HALLMARKS_ROOT}"  || { echo "_RBGA_HALLMARKS_ROOT missing"  >&2; exit 1; }
test -n "${_RBGA_HALLMARK}"        || { echo "_RBGA_HALLMARK missing"        >&2; exit 1; }
test -n "${_RBGA_BIND_SOURCE}"     || { echo "_RBGA_BIND_SOURCE missing"     >&2; exit 1; }

DEST_REF="${_RBGA_GAR_HOST}/${_RBGA_GAR_PATH}/${_RBGA_HALLMARKS_ROOT}/${_RBGA_HALLMARK}/image:${_RBGA_HALLMARK}"

echo "=== Mirroring bind image via gcrane ==="
echo "Source: ${_RBGA_BIND_SOURCE}"
echo "Dest:   ${DEST_REF}"

# gcrane authenticates GAR ambiently through its google.Keychain
# (ADC -> GCE metadata server, the Mason SA). No --creds needed.
gcrane cp "${_RBGA_BIND_SOURCE}" "${DEST_REF}" \
  || { echo "gcrane cp failed" >&2; exit 1; }

echo "Image mirrored: ${DEST_REF}"
echo "=== Mirror step complete ==="
