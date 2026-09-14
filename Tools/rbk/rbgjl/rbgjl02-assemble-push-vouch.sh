#!/bin/bash
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
# RBGJL Step 02: Push each Lode's provenance envelope as its :rbi_vouch artifact
# Builder: gcrane (pinned reliquary gcrane for bole/wsl/podvm; floating bootstrap
#          gcrane for conclave — the recipe row in each per-kind body sets which)
# Entrypoint: busybox (gcrane:debug's only shell)
# Substitutions: _RBGL_GAR_HOST, _RBGL_GAR_PATH, _RBGL_LODES_ROOT, _RBGL_TAG_VOUCH,
#                _RBGL_GIT_COMMIT (spine-injected, never in a body's blob)
#
# Note: this script runs inside a Cloud Build container, not under BCG module
# discipline (CBG governs).
#
# The capture step (rbgjl01 ensconce / rbgjl03 conclave / rbgjl04 underpin / rbgjl07
# immure) staged one envelope per captured Lode at /workspace/lode_<stamp>_vouch.json
# and listed the stamps in /workspace/lode_stamps.txt. Each envelope rides into the SAME package as
# the captured manifest, under the :rbi_vouch tag — a distinct manifest, one per Lode.
# Vouch content is architecture-independent; the single appended layer is sufficient.

set -euo pipefail

echo "=== Assemble and push Lode vouch artifacts ==="

test -f /workspace/lode_stamps.txt \
  || { echo "FATAL: /workspace/lode_stamps.txt not found — step 01 must run first" >&2; exit 1; }
test -s /workspace/lode_stamps.txt \
  || { echo "FATAL: /workspace/lode_stamps.txt is empty — nothing ensconced" >&2; exit 1; }

test -n "${_RBGL_TAG_VOUCH}" || { echo "FATAL: _RBGL_TAG_VOUCH missing" >&2; exit 1; }
test -n "${_RBGL_GIT_COMMIT}" || { echo "FATAL: _RBGL_GIT_COMMIT missing" >&2; exit 1; }

while IFS= read -r STAMP || test -n "${STAMP}"; do
  test -n "${STAMP}" || continue

  ENVELOPE_FILE="/workspace/lode_${STAMP}_vouch.json"
  test -f "${ENVELOPE_FILE}" \
    || { echo "FATAL: envelope not staged for ${STAMP}: ${ENVELOPE_FILE}" >&2; exit 1; }

  VOUCH_URI="${_RBGL_GAR_HOST}/${_RBGL_GAR_PATH}/${_RBGL_LODES_ROOT}/${STAMP}:${_RBGL_TAG_VOUCH}"
  echo "--- Vouch for ${STAMP} -> ${VOUCH_URI} ---"

  # Stage the envelope alone in a context dir; gcrane append wraps it as a FROM-scratch
  # single-layer member (vouch.json lands at image root).
  CTX="/workspace/vouch_ctx_${STAMP}"
  mkdir -p "${CTX}"

  # Splice the dispatching HEAD commit into the envelope — this shared push step is
  # the one injection point every kind rides, so the capture steps never author the
  # field. Pure-shell prepend inside the leading brace (busybox has no jq); the
  # staged envelope is a single-line JSON object and the commit is bare hex, so no
  # quoting hazard. The GAR :rbi_vouch artifact is the envelope's only canonical
  # home; the pre-splice staged copy is workspace debris.
  ENVELOPE_JSON=$(cat "${ENVELOPE_FILE}")
  case "${ENVELOPE_JSON}" in
    "{"*) ;;
    *) echo "FATAL: staged envelope for ${STAMP} is not a JSON object" >&2; exit 1 ;;
  esac
  printf '{"rblv_git_commit":"%s",%s' "${_RBGL_GIT_COMMIT}" "${ENVELOPE_JSON#\{}" > "${CTX}/vouch.json"

  # Push the FROM-scratch vouch layer via gcrane append — Lode-family snippet.
  APPEND_CTX="${CTX}"
  APPEND_URI="${VOUCH_URI}"
#@rbgjs_include gcrane-append

  echo "Vouch pushed: ${VOUCH_URI}"
done < /workspace/lode_stamps.txt

echo "=== Vouch push step complete ==="
