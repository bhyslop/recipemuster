#!/bin/bash
set -e

# Start SSH daemon
echo "Starting SSH daemon..."
/usr/sbin/sshd -D &
SSH_PID=$!

# Verify Claude Code installation
if [ -x "/usr/local/bin/claude-code" ]; then
    echo "Claude Code is installed at /usr/local/bin/claude-code"
    /usr/local/bin/claude-code --version || echo "Version check failed (may need authentication)"
else
    echo "Warning: Claude Code not found or not executable"
fi

# Export environment variables if they exist
if [ -n "$ANTHROPIC_API_KEY" ]; then
    export ANTHROPIC_API_KEY
    echo "ANTHROPIC_API_KEY is set"
fi

echo "Container ready. SSH available on port 22 (mapped to host port 8888)"
echo "Connect with: ssh -p 8888 claude@localhost"

# Keep container running
wait $SSH_PID

