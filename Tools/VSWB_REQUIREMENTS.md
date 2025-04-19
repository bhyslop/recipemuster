# SlickEdit Workspace Builder (VSWB) Requirements - Revised

## Overview

The SlickEdit Workspace Builder (VSWB) is a set of bash functions that generate SlickEdit project files (.vpj) and workspace files (.vpw) from a script-based specification.

## Components

1. **Builder Script (`vswb_builder.sh`)**: Core functions for defining projects, file patterns, and workspaces
2. **Instance Scripts**: User-created scripts that use the builder functions
3. **Generated Files**: The .vpj and .vpw files created by the builder

## Path Structure

The VSWB system handles paths with three distinct components:

1. **Repository Parent Path** (`repo_parent_path`): Relative path from the SlickEdit project files to the parent directory of the repository (e.g., `../..`)
2. **Repository Name** (`repo_name`): Name of the repository directory (e.g., `usi-UtilitiesScaleInvariant`)
3. **Repository Subpath** (`repo_subpath`): Path inside the repository (e.g., `tt`, `app-hjtest`, or empty for root)

Complete file paths are constructed by combining: `repo_parent_path` + `/` + `repo_name` + `/` + `repo_subpath`

## Script Robustness Requirements

1. **Error Handling**: The script uses `set -e` to terminate immediately if any command fails
2. **Descriptive Echo Statements**: The script uses descriptive echo statements instead of comments for better debugging
3. **File Output Method**: The script uses serial `echo "xxx" >> file` statements rather than HERE documents
4. **Parameter Validation**: Functions validate parameters and fail with clear error messages when invalid

## API Reference

### Core Functions

#### `vswb_define_project`
Defines a new project that will be stored in a .vpj file.

**Parameters:**
- `project_id`: Unique identifier for the project
- `project_name`: Display name for the project

**Validation:**
- `project_id`: Non-empty string
- `project_name`: Non-empty string

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

**Validation:**
- `project_id`: References a previously defined project
- `repo_subpath`: Valid string (can be empty)
- `file_pattern`: Non-empty string
- `excludes`: Valid string (can be empty)

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

**Validation:**
- `project_id`: References a previously defined project
- `repo_subpath`: Valid string (can be empty)
- `file_pattern`: Non-empty string
- `excludes`: Valid string (can be empty)

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

**Validation:**
- `project_id`: References a previously defined project
- `repo_subpath`: Valid string (can be empty)
- `file_name`: Non-empty string

**Example:**
```bash
vswb_add_specific_file "c-files" "" "specific-file.c"
```

#### `vswb_init_workspace`
Initializes a workspace (.vpw) file and associates it with projects.

**Parameters:**
- `workspace_id`: Unique identifier for the workspace
- `workspace_name`: Display name for the workspace
- `project_id1 project_id2 ...`: List of project IDs to include in the workspace

**Validation:**
- `workspace_id`: Non-empty string
- `workspace_name`: Non-empty string
- Each project ID: References a previously defined project

**Example:**
```bash
vswb_init_workspace "all" "my-ALL" "c-files" "cpp-files" "python-files" "infrastructure"
```

#### `vswb_generate_files`
Generates all SlickEdit project (.vpj) and workspace (.vpw) files.

**Parameters:**
- `output_directory`: Directory where files will be created
- `repo_parent_path`: Relative path to the parent of the repository
- `repo_name`: Name of the repository directory
- `prefix`: Prefix to use for all generated workspace and project files

**Validation:**
- `output_directory`: Non-empty string
- `repo_parent_path`: Non-empty string
- `repo_name`: Non-empty string
- `prefix`: Non-empty string
- No filesystem validation is performed for these paths

**Note:** This function creates the target directory if it doesn't exist.

**Example:**
```bash
vswb_generate_files "./output" "../.." "my-project" "myproj"
```

## Boilerplate Files

The script looks for boilerplate files in the same directory as the script itself:

1. **Top Boilerplate**: `$(dirname "${BASH_SOURCE[0]}")/vswb.boilerplate-top.txt`
2. **Bottom Boilerplate**: `$(dirname "${BASH_SOURCE[0]}")/vswb.boilerplate-bottom.txt`

These files provide standard content for the beginning and end of the generated project files.

## Output File Structure

The generated files follow this naming convention:
- Project files: `{prefix}-{project_id}.vpj`
- Workspace files: `{prefix}-{workspace_id}.vpw`

Where `{prefix}` is explicitly provided in the `vswb_generate_files` function.

## Usage Example

```bash
#!/bin/bash
source "$(dirname "${BASH_SOURCE[0]}")/vswb_builder.sh"

# Set script to exit on first error
set -e

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
vswb_generate_files "./output" "../.." "my-project" "myproj"
```

## Notes

1. All paths in `vswb_add_*` functions are relative to the repository root
2. The actual file paths in generated .vpj files are constructed by combining the repository parent path, repository name, and repository subpath
3. Multiple workspaces can include the same project files
4. The Builder script handles directory creation for output files
