#!/bin/bash
set -e

# Ensure claude user home is ready
CLAUDE_HOME=$(getent passwd claude | cut -d: -f6)
chown -R claude:claude "${CLAUDE_HOME}"

# Start SSH daemon in foreground (container lifecycle tied to sshd)
exec /usr/sbin/sshd -D
