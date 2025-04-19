#!/bin/bash
#
# SlickEdit Workspace Builder (VSWB) - Builder Script
# Implementation based on VSWB_REQUIREMENTS.md
#

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
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
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
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
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
    
    # Increment and get unique pattern ID
    FILE_PATTERN_COUNTER=$((FILE_PATTERN_COUNTER + 1))
    local pattern_id="${project_id}_${FILE_PATTERN_COUNTER}"
    
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
    
    # Store workspace metadata and associated projects
    WORKSPACES["$workspace_id"]="${workspace_name}|${project_ids}"
}

# Generate SlickEdit project and workspace files
# Parameters:
#   $1: output_directory - Directory where files will be created
#   $2: repo_parent_path - Relative path to the parent of the repository
#   $3: repo_name - Name of the repository directory
vswb_generate_files() {
    local output_directory="$1"
    local repo_parent_path="$2"
    local repo_name="$3"
    
    # Create output directory if it doesn't exist
    mkdir -p "${output_directory}"
    
    # Get boilerplate content
    local boilerplate_top=""
    local boilerplate_bottom=""
    
    if [ -f "Tools/vswb.boilerplate-top.txt" ]; then
        boilerplate_top=$(cat "Tools/vswb.boilerplate-top.txt")
    fi
    
    if [ -f "Tools/vswb.boilerplate-bottom.txt" ]; then
        boilerplate_bottom=$(cat "Tools/vswb.boilerplate-bottom.txt")
    fi
    
    # Generate project files
    for project_id in "${!PROJECTS[@]}"; do
        # Extract prefix from project_id
        local prefix=$(echo "${project_id}" | cut -d '-' -f1)
        if [ -z "$prefix" ]; then
            prefix="vswb"
        fi
        
        # Project file path
        local project_file_path="${output_directory}/${prefix}-${project_id}.vpj"
        
        # Generate project file
        echo "${boilerplate_top}" > "${project_file_path}"
        
        # Add files section
        echo "    <Files AutoFolders=\"PackageView\">" >> "${project_file_path}"
        
        # Group file patterns by repo_subpath
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
    done
    
    # Generate workspace files
    for workspace_id in "${!WORKSPACES[@]}"; do
        IFS='|' read -r workspace_name project_list <<< "${WORKSPACES[$workspace_id]}"
        
        # Extract prefix from workspace_name
        local prefix=$(echo "${workspace_name}" | cut -d '-' -f1)
        
        # If no hyphen or if the prefix is empty, use the workspace name
        if [ -z "$prefix" ] || [[ "$workspace_name" != *-* ]]; then
            prefix="$workspace_name"
        fi
        
        # Format the prefix to lowercase
        prefix=$(echo "$prefix" | tr '[:upper:]' '[:lower:]')
        
        # Workspace file path
        local workspace_file_path="${output_directory}/${prefix}-${workspace_id}.vpw"
        
        # Generate workspace file
        cat > "${workspace_file_path}" << EOL
<!DOCTYPE Workspace SYSTEM "http://www.slickedit.com/dtd/vse/10.0/vpw.dtd">
<Workspace Version="10.0" VendorName="SlickEdit">
	<Projects>
EOL
        
        # Add project references
        for project_id in $project_list; do
            # Use the same prefix naming convention as for project files
            local project_prefix=$(echo "${project_id}" | cut -d '-' -f1)
            if [ -z "$project_prefix" ]; then
                project_prefix="vswb"
            fi
            
            echo "        <Project File=\"${prefix}-${project_id}.vpj\"/>" >> "${workspace_file_path}"
        done
        
        # Close workspace file
        cat >> "${workspace_file_path}" << EOL
	</Projects>
</Workspace>
EOL
    done
    
    echo "Generated files in ${output_directory}/"
}

