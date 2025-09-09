#!/bin/bash

# Start SSH daemon
echo "Starting SSH daemon..."
/usr/sbin/sshd -D &
SSH_PID=$!

# Verify Claude Code installation
echo "Verifying Claude Code installation..."
if [ -x /usr/local/bin/claude-code ]; then
    echo "Claude Code is installed at /usr/local/bin/claude-code"
    /usr/local/bin/claude-code --version 2>/dev/null || echo "Claude Code version check failed (may need authentication)"
else
    echo "ERROR: Claude Code not found or not executable"
    exit 1
fi

# Export environment variables for Claude Code
if [ -f /workspace/MyRepo/CLAUDE.md ]; then
    echo "Claude instructions file found at /workspace/MyRepo/CLAUDE.md"
fi

# Export ANTHROPIC_API_KEY if set
if [ -n  "$ANTHROPIC_API_KEY" ]; then
    export ANTHROPIC_API_KEY
    echo  "ANTHROPIC_API_KEY is configured"
else
    echo "WARNING: ANTHROPIC_API_KEY is not set"
fi

# Set up environment for SSH sessions
CLAUDE_HOME=$(getent passwd claude | cut -d: -f6)
echo "export PATH=/usr/local/bin:\$PATH"              >> ${CLAUDE_HOME}/.bashrc
echo "export ANTHROPIC_API_KEY=$ANTHROPIC_API_KEY"    >> ${CLAUDE_HOME}/.bashrc
echo "cd /workspace"                                  >> ${CLAUDE_HOME}/.bashrc

# Ensure proper permissions
chown -R claude:claude ${CLAUDE_HOME}

echo "Container is ready. SSH access available on port 22"
echo "Connect using: ssh -p 8888 claude@localhost"

# Keep container running
wait $SSH_PID

