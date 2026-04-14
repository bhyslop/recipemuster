#!/bin/bash
set -e

# Ensure claude user home is ready
CLAUDE_HOME=$(getent passwd claude | cut -d: -f6)

# Seed persistent workspace with sample project on first boot.
# The volume is empty on initial charge; subsequent charges reuse it.
WORKSPACE="${CLAUDE_HOME}/workspace"
if [ -d /opt/ccyolo-sample ] && [ -d "${WORKSPACE}" ] && [ -z "$(ls -A "${WORKSPACE}" 2>/dev/null)" ]; then
  cp -a /opt/ccyolo-sample/. "${WORKSPACE}/"
  chmod +x "${WORKSPACE}"/*.sh 2>/dev/null || true
fi

chown -R claude:claude "${CLAUDE_HOME}"

# Start SSH daemon in foreground (container lifecycle tied to sshd)
exec /usr/sbin/sshd -D
