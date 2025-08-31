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
gadf_since_days="7"
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
        --since-days)
            gadf_since_days="${2:-}"
            test -n "${gadf_since_days}" || gadf_fail "Error: --since-days requires a number"
            [[ "${gadf_since_days}" =~ ^[0-9]+$ ]] || gadf_fail "Error: --since-days must be a positive number"
            shift 2
            ;;
        --once)
            gadf_once="true"
            shift
            ;;
        *)
            gadf_fail "Error: Unknown argument '${1}'. Usage: --file <filename> --directory <path> [--branch <name>] [--since-days <number>] [--once]"
            ;;
    esac
done

# Validate required arguments
test -n "${gadf_adoc_filename}" || gadf_fail "Error: --file is required"
test -n "${gadf_directory}" || gadf_fail "Error: --directory is required"

# Convert paths to absolute before changing directories
gadf_adoc_filename="$(realpath "${gadf_adoc_filename}")"
gadf_directory="$(realpath "${gadf_directory}" 2>/dev/null || echo "$(pwd)/${gadf_directory}")"

# Capture absolute path to distill script before changing directories
gadf_script_dir="$(dirname "${BASH_SOURCE[0]}")"
gadf_distill_script="$(realpath "${gadf_script_dir}/gadp_distill.py")"

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
gadf_memory_file="${gadf_directory}/.factory-state"

gadf_step "Creating GAD directory structure"
mkdir -p "${gadf_extract_dir}" "${gadf_distill_dir}" "${gadf_output_dir}"

# Clean workspace per single-threaded premise
gadf_step "Cleaning workspace"
rm -rf "${gadf_extract_dir:?}"/* "${gadf_distill_dir:?}"/* 2>/dev/null || true

# Read last processed commit
gadf_last_commit=""
if [[ -f "${gadf_memory_file}" ]]; then
    gadf_last_commit="$(cat "${gadf_memory_file}")"
fi

gadf_step "Discovering commits on branch '${gadf_branch}' since ${gadf_since_days} days ago"

# Get commit list since last processed commit or since-days
gadf_commit_args=()
if [[ -n "${gadf_last_commit}" ]]; then
    gadf_commit_args=("${gadf_last_commit}..${gadf_branch}")
else
    gadf_commit_args=("--since=${gadf_since_days} days ago" "${gadf_branch}")
fi

# Get commits in reverse order (oldest first for processing)
mapfile -t gadf_commits < <(git log --reverse --format="%H" "${gadf_commit_args[@]}")

if [[ ${#gadf_commits[@]} -eq 0 ]]; then
    gadf_step "No new commits to process"
    exit 0
fi

gadf_step "Processing ${#gadf_commits[@]} commits"

# Process each commit
for gadf_commit in "${gadf_commits[@]}"; do
    gadf_step "Processing commit ${gadf_commit:0:8}"
    
    # Clean temporary directories
    rm -rf "${gadf_extract_dir:?}"/* "${gadf_distill_dir:?}"/* 2>/dev/null || true
    
    # Extract commit files via git archive
    git archive "${gadf_commit}" | tar -xf - -C "${gadf_extract_dir}" || gadf_fail "Failed to extract commit ${gadf_commit}"
    
    # Check if AsciiDoc file exists in this commit
    gadf_extracted_adoc="${gadf_extract_dir}/${gadf_adoc_relpath}"
    if [[ ! -f "${gadf_extracted_adoc}" ]]; then
        gadf_warn "AsciiDoc file '${gadf_adoc_relpath}' not found in commit ${gadf_commit}, skipping"
        continue
    fi
    
    # Run asciidoctor
    gadf_step "Running asciidoctor on ${gadf_adoc_filename}"
    asciidoctor -a reproducible "${gadf_extracted_adoc}" -D "${gadf_distill_dir}" || gadf_fail "Asciidoctor failed for commit ${gadf_commit}"
    
    # Get commit metadata
    gadf_commit_timestamp="$(git log -1 --format="%cd" --date=format:"%Y%m%d%H%M%S" "${gadf_commit}")"
    gadf_commit_message="$(git log -1 --format="%s" "${gadf_commit}")"
    
    # Call Python distill with AsciiDoc filename
    gadf_step "Distilling HTML for commit ${gadf_commit:0:8}"
    python "${gadf_distill_script}" \
        "${gadf_distill_dir}" \
        "${gadf_output_dir}" \
        "${gadf_branch}" \
        "${gadf_commit}" \
        "${gadf_commit_timestamp}" \
        "${gadf_commit_message}" \
        "${gadf_adoc_filename}" \
        || gadf_fail "Python distill failed for commit ${gadf_commit}"
    
    # Update memory file
    echo -n "${gadf_commit}" > "${gadf_memory_file}"
    
    # Exit after first commit if --once specified
    if [[ "${gadf_once:-false}" == "true" ]]; then
        gadf_step "Exiting after single commit (--once mode)"
        break
    fi
done

# Copy inspector to output directory for self-contained results
gadf_inspector_source="$(dirname "${BASH_SOURCE[0]}")/gadi_inspector.html"
if [[ -f "${gadf_inspector_source}" ]]; then
    gadf_step "Copying inspector to output directory"
    cp "${gadf_inspector_source}" "${gadf_output_dir}/"
else
    gadf_warn "Inspector file not found at ${gadf_inspector_source}"
fi

gadf_step "GAD factory processing complete. Output in ${gadf_output_dir}"

# eof