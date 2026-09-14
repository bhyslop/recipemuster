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

set -e

# Align container user UID/GID with host when bind-mounting workspace.
# RBOB_HOST_UID/GID arrive via compose environment from rbob_bottle.sh.
# Vessels without bind mounts simply don't set these — the block is skipped.
if [ -n "${RBOB_HOST_UID:-}" ] && [ -n "${RBRV_USER:-}" ]; then
  usermod  -o -u "${RBOB_HOST_UID}" "${RBRV_USER}"
  groupmod -o -g "${RBOB_HOST_GID}" "${RBRV_USER}"
fi

CLAUDE_HOME=$(getent passwd claude | cut -d: -f6)
chown -R claude:claude "${CLAUDE_HOME}"

# Start SSH daemon in foreground (container lifecycle tied to sshd)
exec /usr/sbin/sshd -D
