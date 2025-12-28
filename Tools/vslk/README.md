# VSLK - Visual SlickEdit Local Kit

A template-based Visual SlickEdit project installer for the brm_recipebottle project.

## Overview

VSLK provides a simple, bash 3.2-compatible way to install SlickEdit project configurations. It follows the template-copy pattern used by other similar projects (like aoc-2025).

Instead of generating project files programmatically, VSLK:
1. Maintains pre-configured SlickEdit project files (`.vpj` and `.vpw`) in `projects/`
2. Provides an installer command (`vslk-i`) that copies them to a project-local destination
3. Integrates with the BUK (Bash Utility Kit) infrastructure via the `vslw_workbench.sh`

## Files

### Project Templates
- `projects/vsrbm-all.vpj` - SlickEdit project file for Recipe Bottle Makefile System (RBM)
- `projects/vsrbm-ALL.vpw` - SlickEdit workspace file grouping RBM projects

### Workbench
- `vslw_workbench.sh` - Main workbench script with installation logic

## Installation

Run the tabtarget to install the SlickEdit project:

```bash
tt/vslk-i.InstallSlickEditProject.sh
```

This will:
1. Delete any existing SlickEdit project directory at `../_vs/brm_recipebottle/`
2. Create a fresh directory
3. Copy project template files into it
4. Report success with file count

## Project Structure

The installer detects the project base name from the current working directory:

```
PWD = /Users/bhyslop/projects/brm_recipebottle
Project base name: brm_recipebottle
Destination: ../_vs/brm_recipebottle/
```

This makes the kit reusable in other projects - each project will have its own destination.

## Configuration

All configuration is hardcoded in `vslw_workbench.sh` using `VSLW_*` constants:

- `VSLW_TEMPLATE_DIR` - Where template files are stored (`Tools/vslk/projects`)
- `VSLW_PROJECT_BASE_NAME` - Detected from current directory (`${PWD##*/}`)
- `VSLW_DEST_DIR` - Computed destination for installed files (`../_vs/${VSLW_PROJECT_BASE_NAME}`)

## Cohorts and Spaces

The SlickEdit project file includes three file cohorts:

| Cohort | Files | Recursive |
|--------|-------|-----------|
| Bash Scripts | `*.sh` | Yes |
| Build Files | `*.mk` | Yes |
| Recipe Files | `*.recipe` | Yes (RBM-recipes/) |

A single workspace groups all cohorts into the "ALL" space.

## Design Philosophy

- **Bash 3.2 Compatible**: Uses only bash parameter expansion and standard utilities
- **Template-Based**: Pre-made project files, no code generation complexity
- **Self-Adapting**: Destination directory name matches project folder
- **Integration-Ready**: Works within BUK infrastructure via launcher + workbench pattern

## Future Enhancements

If you need to support multiple IDE backends (VS Code, IntelliJ), consider:
1. Creating separate kits: `Tools/vscode/`, `Tools/jetbrains/`, etc.
2. Or upgrading to YETI if you upgrade bash to 4.x (for code generation approach)

For now, VSLK provides a lightweight, reliable solution for SlickEdit on macOS.
