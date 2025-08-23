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

The following utilities are installed in the container based on the specific requirements discussed:

#### Core Dependencies
- **nodejs** - Required runtime for Claude Code (version 18+ needed)
- **npm** - Package manager for Claude Code installation (comes with nodejs)
- **curl** - For downloading Claude Code installer script and making API calls
- **openssh-server** - Required for SSH access to the container

#### Development Tools
- **git** - For version control operations
- **jq** - For JSON parsing in scripts
- **make** - For build automation
- **vim** - Basic text editor for debugging/inspection
- **ripgrep** - Fast search tool (Claude Code may use this)

#### Python
- **python3** - Python 3.12.3 (default in Ubuntu 24.04)
- **python-is-python3** - Optional symlink for /usr/bin/python compatibility

Note: Python 3.11 specifically is not available in Ubuntu 24.04 default repositories at this time.

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
- Download and install using official installer script: `curl -fsSL https://claude.ai/install.sh | bash -s 1.0.89`
- Pin to specific version 1.0.89 (released August 22, 2025)
- Install to `/usr/local/bin/claude-code`
- Ensure executable permissions
- Initial Claude Code authentication and setup is manual and out of scope for the Docker build

### SSH Configuration
- Configure SSH daemon for passwordless authentication
- Disable root login
- Listen on port 22 (internally, mapped to 8888 externally)
- Generate host keys during build (not runtime)
- Set `PermitEmptyPasswords yes` for user `claude` - This is acceptable and required given the container's internal-only network access and is not a security risk in this isolated environment

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
- Passwordless SSH is explicitly required and acceptable given internal-only access - this is a deliberate design choice for friction-free development within the isolated container environment
- API key stored in ../secrets/CCBX_CLAUDE.env

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
