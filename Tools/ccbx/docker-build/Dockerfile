FROM ubuntu:24.04

# Build arguments for user ID/GID
ARG USER_UID=1000
ARG USER_GID=1000

# Install required packages
RUN apt-get update && apt-get install -y                                                 \
    nodejs              `# Required runtime for Claude Code`                             \
    npm                 `# Package manager for Claude Code installation`                 \
    curl                `# For downloading Claude Code installer and API calls`          \
    openssh-server      `# Required for SSH access to the container`                     \
    git                 `# For version control operations`                               \
    jq                  `# For JSON parsing in scripts`                                  \
    make                `# For build automation`                                         \
    vim                 `# Basic text editor for debugging/inspection`                   \
    ripgrep             `# Fast search tool (Claude Code may use this)`                  \
    python3             `# Python 3.12.3 (default in Ubuntu 24.04)`                      \
    python-is-python3   `# Optional symlink for /usr/bin/python compatibility`           \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user with configurable UID/GID
# Ubuntu 24.04 has ubuntu:1000:1000 by default, so we reuse or override
RUN if getent group ${USER_GID} >/dev/null 2>&1; then                               \
        useradd -m -u ${USER_UID} -g ${USER_GID} -s /bin/bash claude 2>/dev/null || \
        usermod -l claude ubuntu;                                                   \
    else                                                                            \
        groupadd -g                  ${USER_GID} claude                          && \
        useradd -m -u ${USER_UID} -g ${USER_GID} -s /bin/bash claude;               \
    fi

# Set empty password for user claude
RUN passwd -d claude

# Generate SSH host keys
RUN ssh-keygen -A

# Configure SSH daemon
RUN mkdir /var/run/sshd                                         && \
    echo "Port 22"                    >> /etc/ssh/sshd_config   && \
    echo "PermitRootLogin no"         >> /etc/ssh/sshd_config   && \
    echo "PermitEmptyPasswords yes"   >> /etc/ssh/sshd_config   && \
    echo "PasswordAuthentication yes" >> /etc/ssh/sshd_config

# Configure PAM for permissive authentication
RUN sed -i 's/@include common-auth/#@include common-auth/' /etc/pam.d/sshd     && \
    echo "auth sufficient pam_permit.so" >> /etc/pam.d/sshd

# Download and install Claude Code v1.0.89
RUN curl -fsSL https://claude.ai/install.sh | bash -s -- 1.0.89                     && \
    cp -r /root/.local/share/claude /usr/local/share/                               && \
    ln -sf   /usr/local/share/claude/versions/1.0.89 /usr/local/bin/claude-code     && \
    chmod +x /usr/local/share/claude/versions/1.0.89

# Set working directory
WORKDIR /workspace

# Copy entrypoint script
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x       /usr/local/bin/entrypoint.sh

# Expose SSH port
EXPOSE 22

# Run as root to support SSH daemon
USER root

# Set entrypoint
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

