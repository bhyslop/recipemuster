#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: GAD Implementation <generated@scaleinvariant.org>
#

set -euo pipefail

# GADS-compliant error handling
gadf_step() {
    echo "${1}"
}

gadf_warn() {
    echo -e "\033[33m${1}\033[0m"
}

gadf_fail() {
    echo -e "\033[31m${1}\033[0m" >&2
    exit 1
}

# Parse command-line arguments
gadf_adoc_filename=""
gadf_directory=""
gadf_branch="main"
gadf_max_unique_commits="5"
gadf_once="false"

while [[ $# -gt 0 ]]; do
    case "${1}" in
        --file)
            gadf_adoc_filename="${2:-}"
            test -n "${gadf_adoc_filename}" || gadf_fail "Error: --file requires a filename"
            shift 2
            ;;
        --directory)
            gadf_directory="${2:-}"
            test -n "${gadf_directory}" || gadf_fail "Error: --directory requires a path"
            shift 2
            ;;
        --branch)
            gadf_branch="${2:-}"
            test -n "${gadf_branch}" || gadf_fail "Error: --branch requires a branch name"
            shift 2
            ;;
        --max-unique-commits)
            gadf_max_unique_commits="${2:-}"
            test -n "${gadf_max_unique_commits}" || gadf_fail "Error: --max-unique-commits requires a number"
            [[ "${gadf_max_unique_commits}" =~ ^[0-9]+$ ]] || gadf_fail "Error: --max-unique-commits must be a positive number"
            shift 2
            ;;
        --once)
            gadf_once="true"
            shift
            ;;
        *)
            gadf_fail "Error: Unknown argument '${1}'. Usage: --file <filename> --directory <path> [--branch <name>] [--max-unique-commits <number>] [--once]"
            ;;
    esac
done

# Validate required arguments
test -n "${gadf_adoc_filename}" || gadf_fail "Error: --file is required"
test -n "${gadf_directory}" || gadf_fail "Error: --directory is required"

# Convert paths to absolute before changing directories
gadf_adoc_filename="$(realpath "${gadf_adoc_filename}")"
gadf_directory="$(realpath "${gadf_directory}" 2>/dev/null || echo "$(pwd)/${gadf_directory}")"

# Capture absolute paths to scripts before changing directories
gadf_script_dir="$(dirname "${BASH_SOURCE[0]}")"
gadf_distill_script="$(realpath "${gadf_script_dir}/gadp_distill.py")"
gadf_inspector_source="$(realpath "${gadf_script_dir}/gadi_inspector.html")"

# Ensure directory exists (create if needed)
mkdir -p "${gadf_directory}"

# Determine repository directory from AsciiDoc file location
gadf_repo_dir="$(dirname "${gadf_adoc_filename}")"
while [[ "${gadf_repo_dir}" != "/" ]]; do
    if [[ -d "${gadf_repo_dir}/.git" ]]; then
        break
    fi
    gadf_repo_dir="$(dirname "${gadf_repo_dir}")"
done

test -d "${gadf_repo_dir}/.git" || gadf_fail "Error: No git repository found for AsciiDoc file '${gadf_adoc_filename}'"
test -f "${gadf_adoc_filename}" || gadf_fail "Error: AsciiDoc file '${gadf_adoc_filename}' not found"

# Change to repository directory for git operations
cd "${gadf_repo_dir}"

# Convert AsciiDoc filename to relative path from repository root
gadf_adoc_relpath="${gadf_adoc_filename#${gadf_repo_dir}/}"

# Initialize directory structure per GADS specification
gadf_extract_dir="${gadf_directory}/.factory-extract"
gadf_distill_dir="${gadf_directory}/.factory-distill"
gadf_output_dir="${gadf_directory}/output"
gadf_manifest_file="${gadf_output_dir}/manifest.json"

gadf_step "Creating GAD directory structure"
mkdir -p "${gadf_extract_dir}" "${gadf_distill_dir}" "${gadf_output_dir}"

# Copy inspector to output directory for self-contained results (per GADS Initial Setup)
if [[ -f "${gadf_inspector_source}" ]]; then
    gadf_step "Copying inspector to output directory"
    cp "${gadf_inspector_source}" "${gadf_output_dir}/"
else
    gadf_fail "Inspector file not found at ${gadf_inspector_source}"
fi

# Delete existing manifest if present (ensures fresh start)
if [[ -f "${gadf_manifest_file}" ]]; then
    gadf_step "Deleting existing manifest for fresh start"
    rm -f "${gadf_manifest_file}"
fi

# Clean workspace per single-threaded premise
gadf_step "Cleaning workspace"
rm -rf "${gadf_extract_dir:?}"/* "${gadf_distill_dir:?}"/* 2>/dev/null || true

# Factory render sequence function
gadf_render() {
    local commit_hash="${1}"
    
    gadf_step "Executing render sequence for commit ${commit_hash:0:8}"
    
    # Clean temporary directories
    rm -rf "${gadf_extract_dir:?}"/* "${gadf_distill_dir:?}"/* 2>/dev/null || true
    
    # Extract commit files via git archive
    git archive "${commit_hash}" | tar -xf - -C "${gadf_extract_dir}" || gadf_fail "Failed to extract commit ${commit_hash}"
    
    # Check if AsciiDoc file exists in this commit
    gadf_extracted_adoc="${gadf_extract_dir}/${gadf_adoc_relpath}"
    if [[ ! -f "${gadf_extracted_adoc}" ]]; then
        gadf_warn "AsciiDoc file '${gadf_adoc_relpath}' not found in commit ${commit_hash}, skipping"
        return
    fi
    
    # Run asciidoctor
    gadf_step "Running asciidoctor"
    asciidoctor -a reproducible "${gadf_extracted_adoc}" -D "${gadf_distill_dir}" || gadf_fail "Asciidoctor failed for commit ${commit_hash}"
    
    # Get commit metadata
    gadf_commit_timestamp="$(git log -1 --format="%cd" --date=format:"%Y%m%d%H%M%S" "${commit_hash}")"
    gadf_commit_message="$(git log -1 --format="%s" "${commit_hash}")"
    
    # Call Python distill
    gadf_step "Running distill script"
    python "${gadf_distill_script}" \
        "${gadf_distill_dir}" \
        "${gadf_output_dir}" \
        "${gadf_branch}" \
        "${commit_hash}" \
        "${gadf_commit_timestamp}" \
        "${gadf_commit_message}" \
        "${gadf_adoc_filename}" \
        || gadf_fail "Python distill failed for commit ${commit_hash}"
}

# Read distinct count from manifest using jq
get_distinct_count() {
    if [[ -f "${gadf_manifest_file}" ]] && command -v jq >/dev/null 2>&1; then
        jq -r '.distinct_sha256_count // 0' "${gadf_manifest_file}" 2>/dev/null || echo "0"
    else
        echo "0"
    fi
}

# Read last processed hash from manifest using jq
get_last_processed() {
    if [[ -f "${gadf_manifest_file}" ]] && command -v jq >/dev/null 2>&1; then
        jq -r '.last_processed_hash // ""' "${gadf_manifest_file}" 2>/dev/null || echo ""
    else
        echo ""
    fi
}


# Initial population: process commits backwards from HEAD until distinct count reaches max
gadf_step "Starting initial population"
gadf_head_commit="$(git rev-parse "${gadf_branch}")"

while IFS= read -r commit_hash; do
    # Check if AsciiDoc file exists in this commit
    if git show "${commit_hash}:${gadf_adoc_relpath}" >/dev/null 2>&1; then
        # Execute render sequence
        gadf_render "${commit_hash}"
        
        # Check distinct count
        current_count="$(get_distinct_count)"
        gadf_step "Current distinct count: ${current_count}"
        
        if [[ "${current_count}" -ge "${gadf_max_unique_commits}" ]]; then
            gadf_step "Reached maximum unique commits (${gadf_max_unique_commits})"
            break
        fi
    fi
done < <(git log --format="%H" "${gadf_branch}" -100)

# Note: last_processed_hash is set automatically by gadp_distill

# Incremental watch mode (unless --once is specified)
if [[ "${gadf_once:-false}" != "true" ]]; then
    gadf_step "Entering incremental watch mode (polling every 3 seconds)"
    
    while true; do
        sleep 3
        
        # Get current HEAD and last processed
        current_head="$(git rev-parse "${gadf_branch}")"
        last_processed="$(get_last_processed)"
        
        if [[ -n "${last_processed}" && "${current_head}" != "${last_processed}" ]]; then
            gadf_step "Detected new commits beyond ${last_processed:0:8}"
            
            # Process each new commit
            while IFS= read -r commit_hash; do
                if [[ -n "${commit_hash}" ]] && git show "${commit_hash}:${gadf_adoc_relpath}" >/dev/null 2>&1; then
                    gadf_render "${commit_hash}"
                fi
            done < <(git log --reverse --format="%H" "${last_processed}..${gadf_branch}")
            
            # Note: last_processed_hash updated automatically by gadp_distill
        fi
    done
else
    gadf_step "Once mode: exiting after initial population"
fi

# Inspector already copied during Initial Setup

gadf_step "GAD factory processing complete. Output in ${gadf_output_dir}"

# eof