# GADMCR: Factory Python Refactor Specification

**To:** Development Team  
**From:** Architecture Review  
**Date:** August 31, 2025  
**Subject:** Resolution of CORS Blocking via Python Factory Architecture

## Problem Statement

The GAD Inspector cannot function when opened directly from the filesystem due to browser CORS restrictions. The `file://` protocol prevents JavaScript from fetching `manifest.json` and HTML diff files, rendering Factory output unusable without manual HTTP server setup.

## Architectural Changes

### Specification Updates to GADS
- Processing order in GADS will be updated to match the HEAD-first-then-interleave model
- Inspector will handle partial manifest gracefully during backfill

### Component Elimination
- **Bash Factory Script**: Removed entirely
- **Standalone Distill Script**: Absorbed as internal function
- **Static File Deployment**: Replaced by served content

### Component Introduction
- **Python Factory Service**: Single long-running process combining:
    - Git repository monitoring
    - Asciidoctor execution orchestration
    - HTML normalization (former Distill functionality)
    - HTTP server on port 8080
    - Server-Sent Events (SSE) for live updates

### Component Preservation
- **Inspector Interface**: Remains largely unchanged except for:
    - Fetches via HTTP instead of file protocol
    - SSE connection for auto-refresh notifications
    - Served directly from source location (no copy to output/)
- **Working Directory Structure**: Maintained with clarification:
    - `.factory-extract/` for git archive output
    - `.factory-distill/` for asciidoctor output
    - `output/` contains only normalized HTML files and `manifest.json`
    - No `gadt_inspector.html` copy in `output/` directory

## Behavioral Changes

### Processing Order
**Current**: Factory processes commits backward from HEAD until reaching target count

**New**: HEAD-first with intelligent interleaving:
1. Process HEAD immediately on startup (user sees latest content first)
2. Maintain parent chain ordered manifest via full rewrite on each update
3. Sort commits by parent chain traversal order regardless of processing order
4. Manifest always contains only fully processed commits - no incomplete states
5. Inspector dropdowns show available commits with no special partial handling needed

### State Management
**Current**: Bash script uses jq to read manifest, Python Distill updates it

**New**: Python Factory directly manages manifest in memory and disk:
- Single process owns all state
- Atomic manifest updates
- No external tool dependencies (jq eliminated)

### Command-Line Arguments
All Bash Factory arguments will port to Python:
- `--file` (AsciiDoc filename)
- `--directory` (working directory)
- `--branch` (Git branch to track)
- `--max-unique-commits` (distinct SHA256 count target)
- `--once` (disable watch mode)
- `--port` (HTTP server port, default 8080)

### Logging Migration
- `gadfl_step` → Python `logging.info()`
- `gadfl_warn` → Python `logging.warning()`
- `gadfl_fail` → Python `logging.error()` + `sys.exit(1)`
- Console output remains primary debugging interface

### Git Integration
**Current**: Bash subprocess calls to git

**New**: Python subprocess calls to git binaries:
- Safer volume mount handling
- Consistent with current security model
- No GitPython library dependency

### Server Model
**Current**: Static files require external HTTP server

**New**: Integrated HTTP server:
- URL paths mirror filesystem exactly: `/output/manifest.json`, `/output/main-*.html`
- Root path `/` serves Inspector HTML from source location (not from output/)
- `/events` endpoint for SSE stream
- Single-user model (no session isolation)

## Migration Impact

### Deployment Changes
- Container runs single Python process instead of Bash
- Port 8080 exposed for web interface
- Volume mounts unchanged (working directory persistence)
- No auto-restart or process supervision initially

### User Experience Improvements
- Open browser to `http://localhost:8080` instead of file path  
- Automatic refresh when new commits detected (via SSE)
- No CORS errors or security workarounds needed
- HEAD commit viewable immediately, historical commits appear as processed
- SSE events are simple refresh signals: `event: refresh data: {"type": "new_commit"}`
- Multi-tab refresh via SSE is acceptable (all tabs update simultaneously)
- Live Inspector development (refresh picks up Inspector source changes)
- Inspector served from source location enables development iteration

### Development Workflow
- Manual process start with console logging
- Working directory remains inspectable for debugging
- Git operations visible through subprocess output
- Same volume mount strategy for container development
- Any subprocess failure (git, asciidoctor) triggers immediate termination
- No retry logic or commit skipping - preserve state for debugging
- Inspector source location configurable via command-line argument

## File Impact Analysis

### Files to Replace
- **`Tools/gad/gadf_factory.sh`** → **`Tools/gad/gadf_factory.py`**
  - Complete rewrite from Bash to Python
  - Integrates current Distill functionality
  - Adds HTTP server and SSE capabilities

- **`Tools/gad/gadp_distill.py`** → **REMOVED**
  - Functionality absorbed into Python Factory as internal methods
  - No standalone script needed

### Files to Update
- **`lenses/gad-GADS-GoogleAsciidocDifferSpecification.adoc`**
  - Update Factory Main Loop section for HEAD-first processing
  - Update component definitions for integrated architecture
  - Update manifest field management (no separate distill)

- **`CLAUDE.md` (both repositories)**
  - **GADF** mapping: `gad/gadf_factory.sh` → `gad/gadf_factory.py`
  - **GADP** mapping: Remove `gad/gadp_distill.py` entry
  - **GADI** mapping: Unchanged (`gad/gadi_inspector.html`)

- **`Tools/rbk_Coordinator.sh`**
  - `gadf-f` command: Update to call Python Factory
  - `gadcf` command: Update to call Python Factory  
  - `gadi-i` command: Update URL from `/gadi_inspector.html` to `/`
  - Add `--port 8080` argument to Factory calls

### Files to Update (Test/Tools)
- **`tt/gadf-f.Factory.sh`** → **`tt/gadfpy-f.Factory.sh`**
  - Dispatches to Python Factory instead of Bash
  - Consider renaming for clarity

- **`tt/gadcf.LaunchFactoryInContainer.sh`** → **`tt/gadcfpy.LaunchFactoryInContainer.sh`**
  - Dispatches to Python Factory
  - Consider renaming for clarity

- **`tt/gadi-i.Inspect.sh`**
  - Update to connect to `http://localhost:8080/` instead of serving files
  - Remove HTTP server startup (Factory provides server)

### Files Unchanged
- **`Tools/gad/gadi_inspector.html`**
  - Core functionality preserved
  - Only changes: fetch URLs and SSE connection addition

- **`lenses/gad-GADP-GoogleAsciidocDifferPlanner.md`**
  - Deferred concepts remain valid
  - May need updates to reflect completed Python Factory

- **`lenses/gad-GADMCR-MemoCorsResolution.md`**
  - Implementation specification preserved as historical record

### Deployment Changes
- Container configurations calling `gadf_factory.sh` → `gadf_factory.py`
- Port 8080 exposure in container definitions
- Volume mounts unchanged (working directory persistence)

### Container Configuration Updates
**Files requiring changes for Python Factory web server:**

- **`Tools/ccbx/.env`**
  ```bash
  # Add after existing CCBX_SSH_PORT
  CCBX_WEB_PORT=8080
  ```

- **`Tools/ccbx/docker-compose.yml`**
  ```yaml
  ports:
    - "${CCBX_SSH_PORT:-8888}:22"
    - "${CCBX_WEB_PORT:-8080}:8080"  # Add this line
  ```

- **`Tools/ccbx/build-context/Dockerfile`**
  ```dockerfile
  # Add after existing EXPOSE 22
  EXPOSE 8080
  ```

- **`Tools/ccbx/requirements.md`**
  - Update "Port mapping" section to document both SSH and web ports
  - Add web server port to security considerations
  - Update usage instructions to include web access via localhost:8080

## Implementation Clarifications

### URL Path Structure
HTTP server paths mirror the filesystem structure exactly:
- `/output/manifest.json` for manifest
- `/output/main-20250103143022-abc123.html` for normalized HTML files  
- `/` serves Inspector HTML from source location (not from output/)

### Manifest Completeness
With HEAD-first processing, the manifest is always valid and complete for processed commits. During backfill:
- Inspector sees HEAD immediately plus any completed historical commits
- No "incomplete" state exists - manifest only contains fully processed entries
- Inspector dropdowns show available commits; no special handling needed

### SSE Event Structure
SSE events are simple refresh signals:
```
event: refresh
data: {"type": "new_commit"}
```
No commit details needed; Inspector fetches updated manifest on receipt.

### Error Recovery
Python Factory maintains GADS error behavior:
- Any subprocess failure (git, asciidoctor) triggers immediate termination
- No retry logic or commit skipping
- Preserve state for debugging

### Inspector Serving
Inspector HTML served directly from source location:
- No copy to output/ directory
- Python HTTP server maps `/` to Inspector source file path
- Enables live development - refresh picks up Inspector changes
- Source location specified via command-line argument or convention

### Component Preservation Update
Working directory structure remains except:
- No `gadt_inspector.html` copy in `output/`
- Only normalized HTML files and `manifest.json` in `output/`

## Implementation Phases

### Phase 1: Core Python Factory
- Port Bash logic to Python main loop
- Integrate Distill as internal function
- Implement manifest management
- Subprocess calls for git and asciidoctor

### Phase 2: HTTP Server Integration
- Serve Inspector and artifacts
- Implement URL routing matching directory structure
- Add SSE endpoint for refresh events

### Phase 3: Inspector Updates
- Add SSE client connection
- Implement auto-refresh on SSE events
- Remove any file:// protocol assumptions

## Non-Goals

- Multi-user support
- Process supervision/auto-restart
- Database storage (filesystem remains source of truth)
- WebSocket bidirectional communication
- Authentication or access control

## Success Criteria

1. Inspector loads without CORS errors
2. Factory processes commits in same order as current system
3. Working directory structure unchanged
4. Manifest format unchanged
5. Container deployment simplified to single process
6. Live refresh works via SSE notifications
