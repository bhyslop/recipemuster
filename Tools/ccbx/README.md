# Claude Code Box Docker - Usage Guide

## Quick Start

### Prerequisites
- Docker and Docker Compose installed
- API key file at `../secrets/CCBX_CLAUDE.env` containing:
  ```
  ANTHROPIC_API_KEY=your-api-key-here
  ```

### Basic Commands

```bash
# Build and start container
docker-compose up -d

# SSH into container
ssh -p 8888 claude@localhost

# Stop container
docker-compose down

# Rebuild after changes
docker-compose build --no-cache
docker-compose up -d
```

## File System Permissions

For proper file permissions between host and container:

```bash
# Linux/macOS
export USER_UID=$(id -u)
export USER_GID=$(id -g)
docker-compose up -d

# Windows (PowerShell)
$env:USER_UID = 1000
$env:USER_GID = 1000
docker-compose up -d
```

## Directory Structure

```
Tools/ccbx/
├── build-context/
│   ├── Dockerfile         # Container definition
│   └── entrypoint.sh      # Startup script
├── docker-compose.yml     # Service configuration
├── .claude-config/        # Persisted config (auto-created)
├── CLAUDE.md             # Instructions for Claude Code
└── README.md             # This file
```

## Volume Mappings

| Host Path | Container Path | Purpose |
|-----------|----------------|---------|
| `../../../` | `/workspace` | Main workspace (parent of repo) |
| `./CLAUDE.md` | `/workspace/MyRepo/CLAUDE.md` | Claude instructions |
| Named volume | `/home/claude/.claude` | Claude configuration |
| Named volume | `/home/claude/.cache` | Claude cache |

## SSH Access

- **Host**: localhost
- **Port**: 8888
- **User**: claude
- **Password**: (none - passwordless)

Example connection:
```bash
ssh -p 8888 claude@localhost
```

## Claude Code Setup

On first run, authenticate Claude Code:

```bash
# SSH into container
ssh -p 8888 claude@localhost

# Run Claude Code authentication
claude-code auth

# Follow prompts to authenticate
```

## Working with Claude Code

```bash
# Inside container via SSH
cd /workspace/MyRepo

# Start Claude Code session
claude-code

# Claude Code commands
claude-code help
claude-code status
```

## Available Tools

The container includes various development and HTML processing tools:

### HTML Processing
- **diff-dom** - DOM tree diffing for HTML comparison
- **html-minifier** - HTML minification
- **prettier** - Code formatting
- **htmlhint** - HTML validation
- **tidy** - Classic HTML formatter

### Documentation
- **asciidoctor** - AsciiDoc to HTML/PDF processing
- **asciidoctor-pdf** - PDF generation from AsciiDoc

### Development
- Standard development tools (git, vim, ripgrep, jq, make)

## Troubleshooting

### Permission Issues
Ensure `USER_UID` and `USER_GID` match your host user:
```bash
id -u  # Your UID
id -g  # Your GID
```

### Container Won't Start
```bash
# Check logs
docker-compose logs claudecodebox

# Verify API key file exists
ls -la ../secrets/CCBX_CLAUDE.env
```

### SSH Connection Refused
```bash
# Verify container is running
docker ps

# Check SSH service inside container
docker exec ClaudeCodeBox service ssh status
```

### Rebuild Container
```bash
# Clean rebuild
docker-compose down
docker-compose build --no-cache
docker-compose up -d
```

## Security Notes

- Container runs SSH on internal port 22, mapped to host port 8888
- Passwordless SSH is intentional for development convenience
- Container network is bridge mode (not exposed beyond host)
- API key stored in separate secrets file

## Maintenance

### Update Claude Code Version
Edit `Dockerfile` and change the version number in:
```dockerfile
RUN curl -fsSL https://claude.ai/install.sh | bash -s -- 1.0.89
```

### Clean Up Volumes
```bash
# Remove named volumes (loses config/cache)
docker volume rm claude-config claude-cache
```

### View Container Logs
```bash
docker-compose logs -f claudecodebox
```

