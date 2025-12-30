# VSLK - Visual SlickEdit Local Kit

A portable, template-based Visual SlickEdit project installer for BUK-based repositories.

## Overview

VSLK provides a simple, bash 3.2-compatible way to install SlickEdit project configurations. It follows the template-copy pattern and integrates with the BUK (Bash Utility Kit) infrastructure.

Instead of generating project files programmatically, VSLK:
1. Reads pre-configured SlickEdit project files (`.vpj` and `.vpw`) from a repo-specific location
2. Provides an installer command (`vslk-i`) that copies them to a project-local destination
3. Integrates with BUK via the `vslw_workbench.sh` workbench

## Kit Files

- `vslw_workbench.sh` - Main workbench script with installation logic
- `README.md` - This file

## Installation

Run the tabtarget to install the SlickEdit project:

```bash
tt/vslk-i.InstallSlickEditProject.sh
```

This will:
1. Delete any existing SlickEdit project directory at `../_vs/{project-name}/`
2. Create a fresh directory
3. Copy project template files into it
4. Report success with file count

## Configuration

VSLK uses a split configuration model for portability:

**Launcher-provided (repo-specific):**
- `VSLW_TEMPLATE_DIR` - Path to directory containing `.vpj` and `.vpw` template files

**Workbench-computed (automatic):**
- `VSLW_PROJECT_BASE_NAME` - Detected from current directory (`${PWD##*/}`)
- `VSLW_DEST_DIR` - Computed destination (`../_vs/${VSLW_PROJECT_BASE_NAME}`)

To use VSLK in a new repo:
1. Copy `Tools/vslk/` to your repo
2. Create a directory with your `.vpj` and `.vpw` template files
3. Set `VSLW_TEMPLATE_DIR` in your launcher to point to that directory
4. Create the tabtarget and launcher following BUK patterns

## Design Philosophy

- **Bash 3.2 Compatible**: Uses only bash parameter expansion and standard utilities
- **Template-Based**: Pre-made project files, no code generation complexity
- **Self-Adapting**: Destination directory name matches project folder
- **Portable**: Kit code is repo-agnostic; configuration comes from launcher
- **Integration-Ready**: Works within BUK infrastructure via launcher + workbench pattern
