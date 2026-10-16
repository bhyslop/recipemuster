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

echo "RBJ: Beginning sandbox entrypoint"

set -e
test "${RBJ_VERBOSE:-0}" -ge 1 && set -x

echo "RBJp0: Validate compose env-file quoting"
: "${RBJE_PROBE:?}"
z_probe_count=0
z_probe_first=""
z_probe_second=""
for z_word in ${RBJE_PROBE}; do
  z_probe_count=$((z_probe_count + 1))
  if test "${z_probe_count}" -eq 1; then z_probe_first="${z_word}";  fi
  if test "${z_probe_count}" -eq 2; then z_probe_second="${z_word}"; fi
done
test "${z_probe_count}" -eq 2      || { echo "FATAL: RBJE_PROBE token count ${z_probe_count}, expected 2 — value: '${RBJE_PROBE:-}'"; exit 1; }
test "${z_probe_first}" = "alpha"  || { echo "FATAL: RBJE_PROBE first token not 'alpha' — value: '${RBJE_PROBE:-}'"; exit 1; }
test "${z_probe_second}" = "bravo" || { echo "FATAL: RBJE_PROBE second token not 'bravo' — value: '${RBJE_PROBE:-}'"; exit 1; }
echo "RBJp0: Compose env-file quoting validated — RBJE_PROBE = '${RBJE_PROBE}'"

echo "RBJp1: Aligning container user with host"
if test -n "${RBOB_HOST_UID:-}" && test -n "${RBRV_USER:-}"; then
  echo "RBJp1: ${RBRV_USER} to UID ${RBOB_HOST_UID}, GID ${RBOB_HOST_GID:-}"
  usermod  -o -u "${RBOB_HOST_UID}" "${RBRV_USER}" || exit 10
  groupmod -o -g "${RBOB_HOST_GID}" "${RBRV_USER}" || exit 11
else
  echo "RBJp1: No host UID or user forwarded, alignment skipped"
fi

echo "RBJp2: Claiming the claude home"
z_claude_home=$(getent passwd claude | cut -d: -f6)
test -n "${z_claude_home}" || { echo "FATAL: No home directory found for user claude"; exit 20; }
echo "RBJp2: Claude home = ${z_claude_home}"
chown -R claude:claude "${z_claude_home}" || exit 21

echo "RBJp3: Sandbox entrypoint complete, handing off to sshd"
exec /usr/sbin/sshd -D
