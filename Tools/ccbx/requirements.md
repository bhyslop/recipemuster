# Claude Code Box Docker Requirements

## Overview
Containerized environment for running Claude Code on Windows via SSH, avoiding Cygwin/Docker terminal compatibility issues.

## Directory Structure
```
Tools/ccbx/
├── requirements.md         # This document
├── docker-build/           # Build context directory
│   ├── Dockerfile
│   └── entrypoint.sh       # SSH setup and Claude Code initialization
├── docker-compose.yml
├── .env                    # Environment variables (gitignored)
├── .claude-config/         # Persisted Claude config (volume mounted)
└── README.md               # Usage instructions
```

## Dockerfile Requirements

### Base Image
- Ubuntu 24.04 LTS (for stability and long-term support)

### User Configuration
- Create non-root user `claude` with home directory `/home/claude`
- Configure passwordless SSH access for user `claude`
- Set appropriate permissions for Claude Code operations

### Required Installs

The following utilities must be installed in the container based on the specific requirements discussed:

- **openssh-server** - Required for SSH access to the container
- **git** - For version control operations
- **curl** - For downloading Claude Code and making API calls
- **jq** - For JSON parsing in scripts
- **make** - For build automation
- **bash** - Shell environment (may already be present in base image)
- **python3.11** - Python runtime as requested, stable version not bleeding edge

### Package Installation
Following the established style guide with inline comments:
```dockerfile
RUN apt-get update && apt-get install -y                                                 \
    «package-xxx»       `# «rationale-xxx»`                                              \
    «package-yyy»       `# «rationale-yyy»`                                              \
    «package-zzz»       `# «rationale-zzz»`                                              \
    && rm -rf /var/lib/apt/lists/*
```

### Claude Code Installation
- Download and install latest stable version (pin to specific version in Dockerfile)
- Install to `/usr/local/bin/claude-code` 
- Ensure executable permissions

### SSH Configuration
- Configure SSH daemon for passwordless authentication
- Disable root login
- Listen on port 22 (internally, mapped to 8888 externally)
- Generate host keys during build
- Set `PermitEmptyPasswords yes` for user `claude`

### Working Directory
- Set WORKDIR to `/workspace`

### Entrypoint
- Custom entrypoint script to:
  - Start SSH daemon
  - Verify Claude Code installation
  - Set up environment variables
  - Keep container running

## Docker Compose Requirements

### Service Configuration
The service `claudecodebox` requires:
- Container name: ClaudeCodeBox
- Build context: ./docker-build directory with Dockerfile
- Port mapping: Host port 8888 to container SSH port 22
- Environment file: ../secrets/CCBX_CLAUDE.env containing ANTHROPIC_API_KEY

### Volume Mounts
- Workspace: Bind mount ../../../ to /workspace (parent of repo directory)
- Claude config: ./CLAUDE.md to /workspace/MyRepo/CLAUDE.md
- Named volumes for persistence:
  - claude-config: Mount to /home/claude/.claude
  - claude-cache: Mount to /home/claude/.cache

### Network Configuration
- Use bridge network named `claude-network`
- Mark as internal (not accessible from outside host)

## Security Considerations
- Container network is internal-only (not exposed beyond host)
- SSH accessible only via localhost:8888
- Passwordless SSH acceptable given internal-only access
- API key stored in ../secrets/CCBX_CLAUDE.env (must be gitignored)

## Usage Instructions
1. Build and start: `docker-compose up -d`
2. SSH access: `ssh -p 8888 claude@localhost`
3. Stop: `docker-compose down`
4. Rebuild after Dockerfile changes: `docker-compose build --no-cache`

## Volume Persistence
- Claude configuration: Named volume `claude-config`
- Claude cache: Named volume `claude-cache`
- Workspace: Bind mount to host filesystem (parent of repo)

## Platform Compatibility
- Use relative paths in docker-compose.yml
- Path separators handled by Docker
- Works on Windows, macOS, and Linux hosts

