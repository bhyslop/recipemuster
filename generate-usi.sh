#!/bin/bash

# Set script to exit on first error
set -e

# Source the VSWB builder script from the same directory as this script
source "$(dirname "${BASH_SOURCE[0]}")/vswb.builder.sh"

echo "Defining SlickEdit projects for USI..."

# Define projects
vswb_define_project "c-files"        "USI C Files"
vswb_define_project "cpp-files"      "USI C++ Files"
vswb_define_project "python-files"   "USI Python Files"
vswb_define_project "console-files"  "USI Console Files"
vswb_define_project "docker-files"   "USI Docker Files"
vswb_define_project "djproto-htm"    "USI DJProto HTML Files"
vswb_define_project "djproto-python" "USI DJProto Python Files"

echo "Adding file patterns to projects..."

# C files project
vswb_add_files_recurse "c-files" "" "*.c" 
vswb_add_files_recurse "c-files" "" "*.h"

# C++ files project
vswb_add_files_recurse "cpp-files" "" "*.cpp"
vswb_add_files_recurse "cpp-files" "" "*.hpp"

# Python files project
vswb_add_files_recurse "python-files" "" "*.py"

# Console files project
vswb_add_files_nonrecurse "console-files" ""                                  "?akefile"
vswb_add_files_nonrecurse "console-files" ""                                  "*.mk"
vswb_add_files_nonrecurse "console-files" ""                                  "*.md"
vswb_add_files_nonrecurse "console-files" ""                                  "*.sh"
vswb_add_files_recurse    "console-files" "tt"                                "*.sh"
vswb_add_files_nonrecurse "console-files" "Tools"                             "*.mk"
vswb_add_files_nonrecurse "console-files" "Tools"                             "*.sh"
vswb_add_files_recurse    "console-files" "Tools/vsep_VisualSlickEditProject" "*.vpw"
vswb_add_files_recurse    "console-files" "Tools/vsep_VisualSlickEditProject" "*.vpj"

# Docker files project
vswb_add_files_recurse "docker-files" "" "*ockerfile"
vswb_add_files_recurse "docker-files" "" "*ocker-compose.yml"

# DJProto HTML files project
vswb_add_files_recurse "djproto-htm" "djproto-play" "*.htm*"

# DJProto Python files project
vswb_add_files_recurse "djproto-python" "djproto-play" "*.py"

echo "Initializing workspace..."

# Initialize workspaces
vswb_init_workspace "ALL" "vsusi" "c-files" "cpp-files" "python-files" "console-files" "docker-files" "djproto-htm" "djproto-python"

echo "Generating SlickEdit files..."

# Generate all files with explicit prefix
vswb_generate_files "./output" "../.." "usi-ref" "vsusi"

echo "SlickEdit workspace generation complete"

