# SlickEdit Workspace Builder (VSWB) Requirements

## Overview

The SlickEdit Workspace Builder (VSWB) is a set of bash functions designed to generate SlickEdit project files (.vpj) and workspace files (.vpw) from a simple script-based specification. This document outlines the requirements and API for the VSWB system.

## Components

1. **Builder Script (`vswb_builder.sh`)**: Contains the core functions for defining projects, file patterns, and workspaces
2. **Instance Scripts**: User-created scripts that use the builder functions to define specific projects and workspaces
3. **Generated Files**: The .vpj and .vpw files created by the builder

## Path Structure

The VSWB system handles paths with three distinct components:

1. **Repository Parent Path** (`repo_parent_path`): Relative path from the SlickEdit project files to the parent directory of the repository (e.g., `../..`)
2. **Repository Name** (`repo_name`): Name of the repository directory (e.g., `usi-UtilitiesScaleInvariant`)
3. **Repository Subpath** (`repo_subpath`): Path inside the repository (e.g., `tt`, `app-hjtest`, or empty for root)

Complete file paths are constructed by combining: `repo_parent_path` + `/` + `repo_name` + `/` + `repo_subpath`

## API Reference

### Core Functions

#### `vswb_define_project`
Defines a new project.

**Parameters:**
- `project_id`: Unique identifier for the project
- `project_name`: Display name for the project

**Example:**
```bash
vswb_define_project "c-files" "C Source Files"
```

#### `vswb_add_files_recurse`
Adds file patterns to a project with recursive directory scanning.

**Parameters:**
- `project_id`: ID of the project to add files to
- `repo_subpath`: Path inside the repository (empty string for root)
- `file_pattern`: File pattern to match (e.g., "*.c", "Makefile")
- `[excludes]`: Optional patterns to exclude

**Example:**
```bash
vswb_add_files_recurse "c-files" "" "*.c" 
vswb_add_files_recurse "python-files" "app-hjtest" "*.py"
```

#### `vswb_add_files_nonrecurse`
Adds file patterns to a project without recursive directory scanning.

**Parameters:**
- `project_id`: ID of the project to add files to
- `repo_subpath`: Path inside the repository (empty string for root)
- `file_pattern`: File pattern to match (e.g., "*.c", "Makefile")
- `[excludes]`: Optional patterns to exclude

**Example:**
```bash
vswb_add_files_nonrecurse "infrastructure" "" "Makefile"
vswb_add_files_nonrecurse "infrastructure" "" "*.mk"
```

#### `vswb_add_specific_file`
Adds a specific file to a project.

**Parameters:**
- `project_id`: ID of the project to add the file to
- `repo_subpath`: Path inside the repository (empty string for root)
- `file_name`: Name of the specific file to add

**Example:**
```bash
vswb_add_specific_file "c-files" "" "specific-file.c"
```

#### `vswb_init_workspace`
Initializes a workspace and associates it with projects.

**Parameters:**
- `workspace_id`: Unique identifier for the workspace
- `workspace_name`: Display name for the workspace
- `project_id1 project_id2 ...`: List of project IDs to include in the workspace

**Example:**
```bash
vswb_init_workspace "all" "my-ALL" "c-files" "cpp-files" "python-files" "infrastructure"
```

#### `vswb_generate_files`
Generates all SlickEdit project and workspace files.

**Parameters:**
- `output_directory`: Directory where files will be created
- `repo_parent_path`: Relative path to the parent of the repository
- `repo_name`: Name of the repository directory

**Example:**
```bash
vswb_generate_files "./output" "../.." "my-project"
```

## Usage Example

```bash
#!/bin/bash
source ./vswb_builder.sh

# Define projects
vswb_define_project "c-files" "C Source Files"
vswb_define_project "cpp-files" "C++ Source Files"
vswb_define_project "python-files" "Python Source"
vswb_define_project "infrastructure" "Build Infrastructure"

# Add file patterns to projects (all paths are relative to repo root)
vswb_add_files_recurse "c-files" "" "*.c"
vswb_add_files_recurse "c-files" "" "*.h"
vswb_add_specific_file "c-files" "" "specific-file.c"

vswb_add_files_recurse "cpp-files" "" "*.cpp"
vswb_add_files_recurse "cpp-files" "" "*.hpp"

vswb_add_files_recurse "python-files" "" "*.py"

vswb_add_files_nonrecurse "infrastructure" "" "Makefile"
vswb_add_files_nonrecurse "infrastructure" "" "*.mk"
vswb_add_files_recurse "infrastructure" "tt" "*.sh"

# Initialize workspaces with their projects
vswb_init_workspace "all" "my-ALL" "c-files" "cpp-files" "python-files" "infrastructure"
vswb_init_workspace "frontend" "my-frontend" "c-files" "cpp-files"
vswb_init_workspace "scripting" "my-scripting" "python-files" "infrastructure"

# Generate all files
vswb_generate_files "./output" "../.." "my-project"
```

## Output File Structure

The generated files will follow this naming convention:
- Project files: `{prefix}-{project_id}.vpj`
- Workspace files: `{prefix}-{workspace_id}.vpw`

Where `{prefix}` is derived from the workspace name.

## Notes

1. All paths in `vswb_add_*` functions are relative to the repository root
2. The actual file paths in generated .vpj files will be constructed by combining the repository parent path, repository name, and repository subpath
3. Multiple workspaces can include the same project files
4. The Builder script should validate that all projects referenced in workspaces are defined

