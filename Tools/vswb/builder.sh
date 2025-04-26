#!/bin/bash
#
# SlickEdit Workspace Builder (VSWB) - Builder Script
# Implementation based on revised VSWB requirements
#

# Exit on any error
set -e

# Global associative arrays for projects, file patterns, and workspaces
declare -A PROJECTS
declare -A FILE_PATTERNS
declare -A WORKSPACES

# Counter for file pattern IDs
FILE_PATTERN_COUNTER=0

# Define a new project
# Parameters:
#   $1: project_id - Unique identifier for the project
#   $2: project_name - Display name for the project
vswb_define_project() {
    local project_id="$1"
    local project_name="$2"
    
    # Validate parameters
    if [ -z "$project_id" ]; then
        echo "ERROR: project_id parameter is required for vswb_define_project"
        exit 1
    fi
    
    if [ -z "$project_name" ]; then
        echo "ERROR: project_name parameter is required for vswb_define_project"
        exit 1
    fi
    
    echo "Defining project: $project_id ($project_name)"
    
    # Store project metadata 
    PROJECTS["$project_id"]="$project_name"
}

# Add file patterns to a project with recursive directory scanning
# Parameters:
#   $1: project_id - ID of the project to add files to
#   $2: repo_subpath - Path inside the repository (empty string for root)
#   $3: file_pattern - File pattern to match (e.g., "*.c", "Makefile")
#   $4: (optional) excludes - Patterns to exclude
vswb_add_files_recurse() {
    local project_id="$1"
    local repo_subpath="$2"
    local file_pattern="$3"
    local excludes="$4"
    
    # Validate parameters
    if [ -z "$project_id" ]; then
        echo "ERROR: project_id parameter is required for vswb_add_files_recurse"
        exit 1
    fi
    
    if [ -z "${PROJECTS[$project_id]}" ]; then
        echo "ERROR: project_id '$project_id' not found - must define project before adding files"
        exit 1
    fi
    
    if [ -z "$file_pattern" ]; then
        echo "ERROR: file_pattern parameter is required for vswb_add_files_recurse"
        exit 1
    fi
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
    echo "Adding recursive pattern '$file_pattern' to project $project_id in path '$repo_subpath'"
    
    # Store file pattern with recursive flag
    FILE_PATTERNS["$pattern_id"]="${project_id}|${repo_subpath}|${file_pattern}|1|${excludes}"
}

# Add file patterns to a project without recursive directory scanning
# Parameters:
#   $1: project_id - ID of the project to add files to
#   $2: repo_subpath - Path inside the repository (empty string for root)
#   $3: file_pattern - File pattern to match (e.g., "*.c", "Makefile")
#   $4: (optional) excludes - Patterns to exclude
vswb_add_files_nonrecurse() {
    local project_id="$1"
    local repo_subpath="$2"
    local file_pattern="$3"
    local excludes="$4"
    
    # Validate parameters
    if [ -z "$project_id" ]; then
        echo "ERROR: project_id parameter is required for vswb_add_files_nonrecurse"
        exit 1
    fi
    
    if [ -z "${PROJECTS[$project_id]}" ]; then
        echo "ERROR: project_id '$project_id' not found - must define project before adding files"
        exit 1
    fi
    
    if [ -z "$file_pattern" ]; then
        echo "ERROR: file_pattern parameter is required for vswb_add_files_nonrecurse"
        exit 1
    fi
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
    echo "Adding non-recursive pattern '$file_pattern' to project $project_id in path '$repo_subpath'"
    
    # Store file pattern with non-recursive flag
    FILE_PATTERNS["$pattern_id"]="${project_id}|${repo_subpath}|${file_pattern}|0|${excludes}"
}

# Add a specific file to a project
# Parameters:
#   $1: project_id - ID of the project to add the file to
#   $2: repo_subpath - Path inside the repository (empty string for root)
#   $3: file_name - Name of the specific file to add
vswb_add_specific_file() {
    local project_id="$1"
    local repo_subpath="$2"
    local file_name="$3"
    
    # Validate parameters
    if [ -z "$project_id" ]; then
        echo "ERROR: project_id parameter is required for vswb_add_specific_file"
        exit 1
    fi
    
    if [ -z "${PROJECTS[$project_id]}" ]; then
        echo "ERROR: project_id '$project_id' not found - must define project before adding files"
        exit 1
    fi
    
    if [ -z "$file_name" ]; then
        echo "ERROR: file_name parameter is required for vswb_add_specific_file"
        exit 1
    fi
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
    echo "Adding specific file '$file_name' to project $project_id in path '$repo_subpath'"
    
    # Store specific file pattern with non-recursive flag
    FILE_PATTERNS["$pattern_id"]="${project_id}|${repo_subpath}|${file_name}|0|"
}

# Initialize a workspace and associate it with projects
# Parameters:
#   $1: workspace_id - Unique identifier for the workspace
#   $2: workspace_name - Display name for the workspace
#   $3+: project_ids - List of project IDs to include in the workspace
vswb_init_workspace() {
    local workspace_id="$1"
    local workspace_name="$2"
    shift 2
    local project_ids="$@"
    
    # Validate parameters
    if [ -z "$workspace_id" ]; then
        echo "ERROR: workspace_id parameter is required for vswb_init_workspace"
        exit 1
    fi
    
    if [ -z "$workspace_name" ]; then
        echo "ERROR: workspace_name parameter is required for vswb_init_workspace"
        exit 1
    fi
    
    if [ -z "$project_ids" ]; then
        echo "ERROR: at least one project_id is required for vswb_init_workspace"
        exit 1
    fi
    
    # Validate that all project IDs exist
    for project_id in $project_ids; do
        if [ -z "${PROJECTS[$project_id]}" ]; then
            echo "ERROR: project_id '$project_id' not found - must define project before adding to workspace"
            exit 1
        fi
    done
    
    echo "Initializing workspace '$workspace_id' ($workspace_name) with projects: $project_ids"
    
    # Store workspace metadata and associated projects
    WORKSPACES["$workspace_id"]="${workspace_name}|${project_ids}"
}

# Generate SlickEdit project and workspace files
# Parameters:
#   $1: output_directory - Directory where files will be created
#   $2: repo_parent_path - Relative path to the parent of the repository
#   $3: repo_name - Name of the repository directory
#   $4: prefix - Prefix to use for generated files
vswb_generate_files() {
    local output_directory="$1"
    local repo_parent_path="$2"
    local repo_name="$3"
    local prefix="$4"
    
    # Validate parameters
    if [ -z "$output_directory" ]; then
        echo "ERROR: output_directory parameter is required for vswb_generate_files"
        exit 1
    fi
    
    if [ -z "$repo_parent_path" ]; then
        echo "ERROR: repo_parent_path parameter is required for vswb_generate_files"
        exit 1
    fi
    
    if [ -z "$repo_name" ]; then
        echo "ERROR: repo_name parameter is required for vswb_generate_files"
        exit 1
    fi
    
    if [ -z "$prefix" ]; then
        echo "ERROR: prefix parameter is required for vswb_generate_files"
        exit 1
    fi
    
    # Create output directory if it doesn't exist
    echo "Creating output directory: $output_directory"
    mkdir -p "${output_directory}"
    
    # Get script directory
    local script_dir="$(dirname "${BASH_SOURCE[0]}")"
    
    # Get boilerplate content from script directory
    local boilerplate_top=""
    local boilerplate_bottom=""

    echo "Reading top boilerplate"
    boilerplate_top=$(cat "${script_dir}/project-xml-top.txt") || { echo "ERROR: Failed to read top"; exit 1; }

    echo "Reading bottom boilerplate"
    boilerplate_bottom=$(cat "${script_dir}/project-xml-bottom.txt") || { echo "ERROR: Failed to read bottom"; exit 1; }

    # Ensure output directory exists
    mkdir -p "$output_directory"
    
    echo "Clearing existing .vpw and .vpj files from $output_directory"
    find "$output_directory" -maxdepth 1 -type f \( -name "*.vpw" -o -name "*.vpj" \) -delete
    echo "Cleanup complete"
    
    # Generate project files
    for project_id in "${!PROJECTS[@]}"; do
        # Project file path
        local project_file_path="${output_directory}/${prefix}-${project_id}.vpj"
        
        echo "Generating project file: ${project_file_path}"
        
        # Generate project file
        echo "${boilerplate_top}" > "${project_file_path}"
        
        # Add files section
        echo "    <Files AutoFolders=\"PackageView\">" >> "${project_file_path}"
        
        # Group file patterns by repo_subpath  
        # FIXED: Declare the array ONCE before building patterns for THIS project
        declare -A GROUPED_PATTERNS
        
        for pattern_id in "${!FILE_PATTERNS[@]}"; do
            IFS='|' read -r pat_project_id repo_subpath file_pattern recursive excludes <<< "${FILE_PATTERNS[$pattern_id]}"
            
            if [ "${pat_project_id}" == "${project_id}" ]; then
                # Create full path for the folder
                local full_path=""
                if [ -z "${repo_subpath}" ]; then
                    full_path="${repo_parent_path}/${repo_name}"
                else
                    full_path="${repo_parent_path}/${repo_name}/${repo_subpath}"
                fi
                
                # Create or append to group
                if [ -z "${GROUPED_PATTERNS[$full_path]}" ]; then
                    GROUPED_PATTERNS["$full_path"]=""
                fi
                
                # Add file pattern to group
                GROUPED_PATTERNS["$full_path"]+="            <F     N=\"${full_path}/${file_pattern}\" Recurse=\"${recursive}\" Excludes=\"${excludes}\"/>\n"
            fi
        done
        
        # Add grouped patterns to project file
        for folder_path in "${!GROUPED_PATTERNS[@]}"; do
            echo "        <Folder Name=\"${folder_path}\">" >> "${project_file_path}"
            echo -e "${GROUPED_PATTERNS[$folder_path]}" >> "${project_file_path}"
            echo "        </Folder>" >> "${project_file_path}"
            echo "" >> "${project_file_path}"
        done
        
        echo "    </Files>" >> "${project_file_path}"
        echo "" >> "${project_file_path}"
        echo "${boilerplate_bottom}" >> "${project_file_path}"
        
        # IMPORTANT: Clear the array for the next project
        unset GROUPED_PATTERNS
    done
    
    # Generate workspace files
    for workspace_id in "${!WORKSPACES[@]}"; do
        IFS='|' read -r workspace_name project_list <<< "${WORKSPACES[$workspace_id]}"
        
        # Workspace file path
        local workspace_file_path="${output_directory}/${prefix}-${workspace_id}.vpw"
        
        echo "Generating workspace file: ${workspace_file_path}"
        
        # Generate workspace file
        echo "<!DOCTYPE Workspace SYSTEM \"http://www.slickedit.com/dtd/vse/10.0/vpw.dtd\">" > "${workspace_file_path}"
        echo "<Workspace Version=\"10.0\" VendorName=\"SlickEdit\">" >> "${workspace_file_path}"
        echo "	<Projects>" >> "${workspace_file_path}"
        
        # Add project references
        for project_id in $project_list; do
            echo "		<Project File=\"${prefix}-${project_id}.vpj\"/>" >> "${workspace_file_path}"
        done
        
        # Close workspace file
        echo "	</Projects>" >> "${workspace_file_path}"
        echo "</Workspace>" >> "${workspace_file_path}"
    done
    
    echo "Generated all files in ${output_directory}/"
}
