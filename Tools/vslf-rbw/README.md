# VSLF-RBW - SlickEdit Project Files for Recipe Bottle Workbench

Repo-specific SlickEdit project and workspace templates for brm_recipebottle.

## Relationship to VSLK

This directory contains the configuration files consumed by the VSLK (Visual SlickEdit Local Kit) installer. VSLK is a portable kit in `Tools/vslk/` that copies these templates to the installation destination.

- **VSLK** (`Tools/vslk/`) - Portable installer logic, can be copied to other repos
- **VSLF-RBW** (`Tools/vslf-rbw/`) - Repo-specific project templates

The launcher sets `VSLW_TEMPLATE_DIR="Tools/vslf-rbw"` to connect them.

## Projects

| File | Contents | Excludes |
|------|----------|----------|
| `vsp-rbw-main.vpj` | Main development code (*.sh, *.mk, *.recipe) | tt/*, .buk/* |
| `vsp-rbw-tabtargets.vpj` | TabTarget scripts | - |
| `vsp-rbw-launchers.vpj` | BUK launcher scripts | - |
| `vsp-rbw-markdown.vpj` | Markdown documentation (*.md) | - |
| `vsp-rbw-asciidoc.vpj` | AsciiDoc specs (*.adoc) | - |

## Workspaces

| File | Projects Included |
|------|-------------------|
| `vsw-rbw-source-only.vpw` | main only |
| `vsw-rbw-ALL.vpw` | main + tabtargets + launchers + markdown + asciidoc |

## Installation

Run `tt/vslk-i.InstallSlickEditProject.sh` to copy these files to `../_vs/brm_recipebottle/`.
