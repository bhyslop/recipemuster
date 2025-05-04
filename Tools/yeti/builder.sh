#!/usr/bin/env bash


#
# YETI Builder: Spaces & Cohorts Conceptual Model
#
# This system organizes files into conceptual groupings for IDEs:
#
# COHORTS are logical collections of related files defined by patterns:
# - Suffix patterns: Match files by their extension (.py, .js, .html)
# - Prefix patterns: Match files by their starting text (Dockerfile*, Test*)
# - Name patterns: Match specific filenames (Makefile, README.md)
# - Deep patterns: Recursively match in subdirectories (src/**/*.py)
# - Flat patterns: Only match in a single directory (scripts/*.sh)
#
# SPACES are named collections of particular Cohorts that form a complete working context.
# A Space can include multiple overlapping Cohorts for focusing on particular tasks.
#
# The builder translates these concepts into IDE-specific workspace configurations:
# - SlickEdit: Direct mapping to project/workspace files
# - IntelliJ:  Project structure with content roots
# - VS Code:   Approximated using folder roots with file exclusion patterns
#
# Public API Specification:
#
#   yeti_cohort           <cohort_id> <cohort_name>                       # Define a new cohort
#   yeti_add_deep_suffix  <cohort> <repo_subpath> <suffix> [<suffix>...]  # Match filename endings recursively
#   yeti_add_flat_suffix  <cohort> <repo_subpath> <suffix> [<suffix>...]  # Match filename endings in single dir
#   yeti_add_deep_prefix  <cohort> <repo_subpath> <prefix> [<prefix>...]  # Match filename beginnings recursively
#   yeti_add_flat_prefix  <cohort> <repo_subpath> <prefix> [<prefix>...]  # Match filename beginnings in single dir
#   yeti_add_deep_name    <cohort> <repo_subpath> <name> [<name>...]      # Match exact filenames recursively
#   yeti_add_flat_name    <cohort> <repo_subpath> <name> [<name>...]      # Match exact filenames in single dir
#   yeti_space            <space_id> <space_name> <cohort> [<cohort>...]  # Define a space from cohorts
#   yeti_generate         <output_dir> <code_dir> <prefix> <rel_path>     # Generate IDE configurations
#
# Usage examples:
#
# Suffix patterns (extensions):
#   yeti_add_deep_suffix  cpp-files  src  .cpp .hpp .h
#   yeti_add_flat_suffix  build-files  .  .mk .md
#
# Prefix patterns (file beginnings):
#   yeti_add_deep_prefix  tests  src  Test Mock
#   yeti_add_flat_prefix  docker  .  Dockerfile
#
# Exact filename patterns:
#   yeti_add_deep_name  config  .  Makefile CMakeLists.txt
#   yeti_add_flat_name  docs  .  README.md CONTRIBUTING.md
#
# Create a space from cohorts:
#   yeti_space ALL "MyProject" cpp-files build-files tests config docs
#
# Generate all IDE configurations:
#   yeti_generate "workspaces" "." "myproj" "../myproject"
#
# VS Code Implementation Note:
#   Workspaces are created using minimal folder roots with file exclusion patterns
#   that hide everything except cohort-defined files. This approach balances
#   precision with resilience to file changes.

set -euo pipefail

# Global associative arrays
declare -A ZYETI_PROJECTS
declare -A ZYETI_WORKSPACES

# Pattern arrays
declare -A ZYETI_SUFFIX_PATTERNS
declare -A ZYETI_PREFIX_PATTERNS
declare -A ZYETI_NAME_PATTERNS

# Filesystem scan arrays (for intellij and vscode)
declare -A ZYETI_FILES_IN_DIR
declare -A ZYETI_PREFIXES_IN_DIR
declare -A ZYETI_PREFIXES_IN_DIR

# Counters for pattern IDs
ZYETI_SUFFIX_COUNTER=0
ZYETI_PREFIX_COUNTER=0
ZYETI_NAME_COUNTER=0

# Define a new Cohort
yeti_cohort() {
  local cohort_id="$1"
  local cohort_name="$2"
  test -z "$cohort_id"   && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "$cohort_name" && { echo "ERROR: cohort_name required"; exit 1; }
  ZYETI_PROJECTS["$cohort_id"]="$cohort_name"
}

# Add one or more recursive (deep) suffix patterns to a cohort
yeti_add_deep_suffix() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one suffix required"; exit 1; }
  for suffix in "$@"; do
    ZYETI_SUFFIX_COUNTER=$((ZYETI_SUFFIX_COUNTER + 1))
    ZYETI_SUFFIX_PATTERNS["${cohort_id}_$ZYETI_SUFFIX_COUNTER"]="${cohort_id}|${repo_subpath}|${suffix}|1"
  done
}

# Add one or more non-recursive (flat) suffix patterns to a cohort
yeti_add_flat_suffix() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one suffix required"; exit 1; }
  for suffix in "$@"; do
    ZYETI_SUFFIX_COUNTER=$((ZYETI_SUFFIX_COUNTER + 1))
    ZYETI_SUFFIX_PATTERNS["${cohort_id}_$ZYETI_SUFFIX_COUNTER"]="${cohort_id}|${repo_subpath}|${suffix}|0"
  done
}

# Add one or more recursive (deep) prefix patterns to a cohort
yeti_add_deep_prefix() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one prefix required"; exit 1; }
  for prefix in "$@"; do
    ZYETI_PREFIX_COUNTER=$((ZYETI_PREFIX_COUNTER + 1))
    ZYETI_PREFIX_PATTERNS["${cohort_id}_$ZYETI_PREFIX_COUNTER"]="${cohort_id}|${repo_subpath}|${prefix}|1"
  done
}

# Add one or more non-recursive (flat) prefix patterns to a cohort
yeti_add_flat_prefix() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one prefix required"; exit 1; }
  for prefix in "$@"; do
    ZYETI_PREFIX_COUNTER=$((ZYETI_PREFIX_COUNTER + 1))
    ZYETI_PREFIX_PATTERNS["${cohort_id}_$ZYETI_PREFIX_COUNTER"]="${cohort_id}|${repo_subpath}|${prefix}|0"
  done
}

# Add one or more recursive (deep) exact filename patterns to a cohort
yeti_add_deep_name() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one name required"; exit 1; }
  for name in "$@"; do
    ZYETI_NAME_COUNTER=$((ZYETI_NAME_COUNTER + 1))
    ZYETI_NAME_PATTERNS["${cohort_id}_$ZYETI_NAME_COUNTER"]="${cohort_id}|${repo_subpath}|${name}|1"
  done
}

# Add one or more non-recursive (flat) exact filename patterns to a cohort
yeti_add_flat_name() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  test -z "$cohort_id"                         && { echo "ERROR: cohort_id required"; exit 1; }
  test -z "${ZYETI_PROJECTS[$cohort_id]:-}"    && { echo "ERROR: unknown cohort_id $cohort_id"; exit 1; }
  test "$#" -lt 1                              && { echo "ERROR: at least one name required"; exit 1; }
  for name in "$@"; do
    ZYETI_NAME_COUNTER=$((ZYETI_NAME_COUNTER + 1))
    ZYETI_NAME_PATTERNS["${cohort_id}_$ZYETI_NAME_COUNTER"]="${cohort_id}|${repo_subpath}|${name}|0"
  done
}

# Legacy support for backward compatibility
yeti_add_deep() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  
  for pattern in "$@"; do
    # Determine if this is a suffix, prefix, or name pattern
    if [[ "$pattern" == *"*."* ]]; then
      # This is a suffix pattern (e.g., "*.cpp")
      local suffix="${pattern#*\.}"
      yeti_add_deep_suffix "$cohort_id" "$repo_subpath" ".$suffix"
    elif [[ "$pattern" == *"*" ]]; then
      # This is a prefix pattern (e.g., "Test*")
      local prefix="${pattern%\*}"
      yeti_add_deep_prefix "$cohort_id" "$repo_subpath" "$prefix"
    else
      # This is an exact name
      yeti_add_deep_name "$cohort_id" "$repo_subpath" "$pattern"
    fi
  done
}

# Legacy support for backward compatibility
yeti_add_flat() {
  local cohort_id="$1"
  local repo_subpath="$2"
  shift 2
  
  for pattern in "$@"; do
    # Determine if this is a suffix, prefix, or name pattern
    if [[ "$pattern" == *"*."* ]]; then
      # This is a suffix pattern (e.g., "*.cpp")
      local suffix="${pattern#*\.}"
      yeti_add_flat_suffix "$cohort_id" "$repo_subpath" ".$suffix"
    elif [[ "$pattern" == *"*" ]]; then
      # This is a prefix pattern (e.g., "Test*")
      local prefix="${pattern%\*}"
      yeti_add_flat_prefix "$cohort_id" "$repo_subpath" "$prefix"
    else
      # This is an exact name
      yeti_add_flat_name "$cohort_id" "$repo_subpath" "$pattern"
    fi
  done
}

# Legacy support for backward compatibility
yeti_add_file() {
  local cohort_id="$1"
  local repo_subpath="$2"
  local file_name="$3"
  
  yeti_add_flat_name "$cohort_id" "$repo_subpath" "$file_name"
}

# Define a Space as a named collection of Cohorts
yeti_space() {
  local space_id="$1"
  local space_name="$2"
  shift 2
  test -z "$space_id"                       && { echo "ERROR: space_id required";               exit 1; }
  test -z "$space_name"                     && { echo "ERROR: space_name required";             exit 1; }
  test "$#" -lt 1                           && { echo "ERROR: at least one cohort_id required"; exit 1; }
  for cid in "$@"; do
    test -n "${ZYETI_PROJECTS[$cid]:-}"     || { echo "ERROR: unknown cohort_id $cid";          exit 1; }
  done
  ZYETI_WORKSPACES["$space_id"]="$space_name|$*"
}

# Internal: generate SlickEdit project and workspace files
zyeti_generate_slickedit() {
  local output_directory="$1"
  local rel_path_to_code="$2"
  local repo_name="$3"
  local prefix="$4"

  echo "YETIGEN: Assuring existence of target dir:   $output_directory"
  mkdir -p "$output_directory" || { echo "ERROR: mkdir failed for $output_directory"; exit 1; }

  local script_dir
  script_dir="$(dirname "${BASH_SOURCE[0]}")"
  local boilerplate_top boilerplate_bottom
  boilerplate_top="$(cat "$script_dir/project-xml-top.txt")"       || { echo "ERROR: reading top boilerplate";    exit 1; }
  boilerplate_bottom="$(cat "$script_dir/project-xml-bottom.txt")" || { echo "ERROR: reading bottom boilerplate"; exit 1; }

  echo "YETIGEN: Clearing prior .vpw and .vpj from   $output_directory"
  find "$output_directory" -maxdepth 1 -type f \( -name "*.vpw" -o -name "*.vpj" \) -delete || { echo "ERROR: clearing old files failed"; exit 1; }

  for project_id in "${!ZYETI_PROJECTS[@]}"; do
    local project_file
    project_file="$output_directory/${prefix}-${project_id}.vpj"
    echo "YETIGEN: Generating SlickEdit project file:  $project_file"
    echo "$boilerplate_top"                                          >  "$project_file"
    echo "    <Files AutoFolders=\"PackageView\">"                   >> "$project_file"

    declare -A GROUPED_PATTERNS=()
    
    # Process suffix patterns
    for pid in "${!ZYETI_SUFFIX_PATTERNS[@]}"; do
      IFS='|' read pat_pid repo_subpath suffix recursive <<< "${ZYETI_SUFFIX_PATTERNS[$pid]}"
      if [ "$pat_pid" = "$project_id" ]; then
        local full_path
        if [ -z "$repo_subpath" ]; then
          full_path="$rel_path_to_code"
        else
          full_path="$rel_path_to_code/$repo_subpath"
        fi
        local pattern="*$suffix"
        GROUPED_PATTERNS["$full_path"]+="            <F     N=\"$full_path/$pattern\" Recurse=\"$recursive\"/>
"
      fi
    done
    
    # Process prefix patterns
    for pid in "${!ZYETI_PREFIX_PATTERNS[@]}"; do
      IFS='|' read pat_pid repo_subpath prefix recursive <<< "${ZYETI_PREFIX_PATTERNS[$pid]}"
      if [ "$pat_pid" = "$project_id" ]; then
        local full_path
        if [ -z "$repo_subpath" ]; then
          full_path="$rel_path_to_code"
        else
          full_path="$rel_path_to_code/$repo_subpath"
        fi
        local pattern="$prefix*"
        GROUPED_PATTERNS["$full_path"]+="            <F     N=\"$full_path/$pattern\" Recurse=\"$recursive\"/>
"
      fi
    done
    
    # Process name patterns
    for pid in "${!ZYETI_NAME_PATTERNS[@]}"; do
      IFS='|' read pat_pid repo_subpath name recursive <<< "${ZYETI_NAME_PATTERNS[$pid]}"
      if [ "$pat_pid" = "$project_id" ]; then
        local full_path
        if [ -z "$repo_subpath" ]; then
          full_path="$rel_path_to_code"
        else
          full_path="$rel_path_to_code/$repo_subpath"
        fi
        local pattern="$name"
        GROUPED_PATTERNS["$full_path"]+="            <F     N=\"$full_path/$pattern\" Recurse=\"$recursive\"/>
"
      fi
    done

    for folder in "${!GROUPED_PATTERNS[@]}"; do
      echo "        <Folder Name=\"$folder\">"                       >> "$project_file"
      printf "%b" "${GROUPED_PATTERNS[$folder]}"                     >> "$project_file"
      echo "        </Folder>"                                       >> "$project_file"
      echo ""                                                        >> "$project_file"
    done

    echo "    </Files>"                                              >> "$project_file"
    echo ""                                                          >> "$project_file"
    echo "$boilerplate_bottom"                                       >> "$project_file"
  done

  for ws in "${!ZYETI_WORKSPACES[@]}"; do
    IFS='|' read ws_name proj_list <<< "${ZYETI_WORKSPACES[$ws]}"
    local ws_file
    ws_file="$output_directory/${prefix}-$ws.vpw"
    echo "YETIGEN: Generating SlickEdit workspace:     $ws_file"
    echo "<!DOCTYPE Workspace SYSTEM \"http://www.slickedit.com/dtd/vse/10.0/vpw.dtd\">" > "$ws_file"
    echo "<Workspace Version=\"10.0\" VendorName=\"SlickEdit\">"     >> "$ws_file"
    echo "    <Projects>"                                            >> "$ws_file"
    for pid in $proj_list; do
      echo "        <Project File=\"${prefix}-$pid.vpj\"/>"          >> "$ws_file"
    done
    echo "    </Projects>"                                           >> "$ws_file"
    echo "</Workspace>"                                              >> "$ws_file"
  done
}

# Internal: generate VS Code .code-workspace files
# zyeti_generate_vscode - Bridging Cohorts to VS Code Workspaces
#
# Function signature:
#   zyeti_generate_vscode(output_directory, source_root, prefix)
#
# Parameters:
#   - output_directory : Directory where workspace files will be generated
#   - source_root      : Root directory containing the source code
#   - prefix           : String prefix for all generated workspace filenames
#   - (implicit)       : Global Spaces, Cohorts, and file patterns from builder state
#
# Conceptual Mapping:
#   This function bridges the Cohort model (precise file collections) to VS Code's
#   folder-based workspace model through extension-based filtering and optimized
#   folder selection.
#
# Execution Flow:
# 1. Directory Analysis
#    • Analyze Cohort patterns to identify root directories
#    • Scan directories to catalog existing file extensions
#    • Determine "allowed" extensions by intersecting scanned extensions with patterns
#
# 2. Workspace Composition (one .code-workspace per Space)
#    • Folders:
#      - Compute minimal set of directories covering all Cohort patterns
#      - Add these to the workspace's "folders" array
#    • files.exclude:
#      - Begin with "**/*": true to hide everything by default
#      - Generate patterns to selectively show Cohort-defined files
#      - Apply recursive/non-recursive patterns appropriately
#      - Handle any user exclusions (via yeti_exclude)
#
# VS Code Integration Notes:
#   - VS Code uses single-pass hide logic with no "include after exclude"
#   - files.exclude applies globally to the entire workspace
#   - Extension-based filtering provides resilience to file additions
#   - Optimization: Avoid per-file enumeration; rely on folder+extension combinations
#
# Regeneration Triggers:
#   - Changes to Cohort definitions
#   - Directory structure changes
#   - New file extensions appearing in project directories
#
# VS Code / Minimatch Nuances:
#   - Single-pass hide logic: setting a pattern to true always hides matches
#   - No include-after-exclude: only never-hidden files remain visible
#   - Global scope: files.exclude applies to the entire workspace
zyeti_generate_vscode() {
  local out_dir="$1"
  local src_root="$2"
  local prefix="$3"

  mkdir -p "$out_dir" || { echo "ERROR: mkdir failed for $out_dir"; exit 1; }
  echo "YETIGEN: Zap prior VS Code workspace from    $out_dir"
  find "$out_dir" -maxdepth 1 -name "${prefix}-*.code-workspace" -delete \
    || echo "WARNING: Failed to clear old .code-workspace files" >&2

  for ws in "${!ZYETI_WORKSPACES[@]}"; do
    IFS='|' read -r ws_name cohort_list <<< "${ZYETI_WORKSPACES[$ws]}"
    local file="$out_dir/${prefix}-$ws.code-workspace"
    echo "YETIGEN: Generating VS Code workspace:       $file"

    # 1) Collect minimal folder roots
    declare -A seen_folders=()
    local folders=()
    for cid in $cohort_list; do
      for pat_type in SUFFIX PREFIX NAME; do
        # Build the array name, e.g. ZYETI_SUFFIX_PATTERNS
        local arr_var="ZYETI_${pat_type}_PATTERNS"
        # Gather all the PIDs
        eval "pids=(\"\${!$arr_var[@]}\")"
        (( ${#pids[@]} == 0 )) && continue

        for pid in "${pids[@]}"; do
          # Pull out the pattern entry
          eval "entry=\"\${$arr_var[\$pid]}\""
          IFS='|' read -r pat_cid repo_subpath _ _ <<< "$entry"
          [[ "$pat_cid" != "$cid" ]] && continue

          local folder_path="$src_root"
          [[ -n "$repo_subpath" ]] && folder_path="$src_root/$repo_subpath"
          # Skip nonexistent or already-seen
          if [[ -d "$folder_path" && -z "${seen_folders[$folder_path]:-}" ]]; then
            seen_folders[$folder_path]=1
            folders+=( "$folder_path" )
          fi
        done
      done
    done

    # 2) Emit JSON
    {
      printf '{\n'
      printf '  "folders": [\n'
      local first=true
      for f in "${folders[@]}"; do
        if $first; then
          printf '    { "path": "%s" }' "$f"
          first=false
        else
          printf ',\n    { "path": "%s" }' "$f"
        fi
      done
      printf '\n  ],\n'
      printf '  "settings": {\n'
      printf '    "files.exclude": {\n'
      printf '      "**/*": true'

      # 3) Build un-hide patterns with dedupe
      declare -A seen_patterns=()
      for cid in $cohort_list; do
        # --- suffix patterns ---
        arr_var="ZYETI_SUFFIX_PATTERNS"
        eval "pids=(\"\${!$arr_var[@]}\")"
        for pid in "${pids[@]}"; do
          eval "entry=\"\${$arr_var[\$pid]}\""
          IFS='|' read -r pat_cid repo_subpath suffix recursive <<< "$entry"
          [[ "$pat_cid" != "$cid" ]] && continue

          if [[ -n "$repo_subpath" ]]; then
            if (( recursive )); then
              pattern="**/$repo_subpath/**/*$suffix"
            else
              pattern="$repo_subpath/*$suffix"
            fi
          else
            if (( recursive )); then
              pattern="**/*$suffix"
            else
              pattern="*$suffix"
            fi
          fi

          key="suffix|$pattern"
          if [[ -z "${seen_patterns[$key]:-}" ]]; then
            seen_patterns[$key]=1
            printf ',\n      "%s": false' "$pattern"
          fi
        done

        # --- prefix patterns ---
        arr_var="ZYETI_PREFIX_PATTERNS"
        eval "pids=(\"\${!$arr_var[@]}\")"
        for pid in "${pids[@]}"; do
          eval "entry=\"\${$arr_var[\$pid]}\""
          IFS='|' read -r pat_cid repo_subpath pref recursive <<< "$entry"
          [[ "$pat_cid" != "$cid" ]] && continue

          if [[ -n "$repo_subpath" ]]; then
            if (( recursive )); then
              pattern="**/$repo_subpath/**/$pref*"
            else
              pattern="$repo_subpath/$pref*"
            fi
          else
            if (( recursive )); then
              pattern="**/$pref*"
            else
              pattern="$pref*"
            fi
          fi

          key="prefix|$pattern"
          if [[ -z "${seen_patterns[$key]:-}" ]]; then
            seen_patterns[$key]=1
            printf ',\n      "%s": false' "$pattern"
          fi
        done

        # --- exact-name patterns ---
        arr_var="ZYETI_NAME_PATTERNS"
        eval "pids=(\"\${!$arr_var[@]}\")"
        for pid in "${pids[@]}"; do
          eval "entry=\"\${$arr_var[\$pid]}\""
          IFS='|' read -r pat_cid repo_subpath name recursive <<< "$entry"
          [[ "$pat_cid" != "$cid" ]] && continue

          if [[ -n "$repo_subpath" ]]; then
            if (( recursive )); then
              pattern="**/$repo_subpath/**/$name"
            else
              pattern="$repo_subpath/$name"
            fi
          else
            if (( recursive )); then
              pattern="**/$name"
            else
              pattern="$name"
            fi
          fi

          key="name|$pattern"
          if [[ -z "${seen_patterns[$key]:-}" ]]; then
            seen_patterns[$key]=1
            printf ',\n      "%s": false' "$pattern"
          fi
        done
      done

      # 4) Close JSON
      printf '\n    }\n'
      printf '  }\n'
      printf '}\n'
    } > "$file"

    echo "YETIGEN: VS Code workspace generated:        $file"
  done
}

# Internal: generate IntelliJ .ipr/.iml files
#==============================================================================
# INTELLIJ IMPLEMENTATION BACKGROUND
#==============================================================================
#
# We evaluated several approaches for generating IntelliJ project files that 
# could selectively include files based on our Cohort patterns. The main 
# challenge was that IntelliJ's project model is fundamentally different from 
# SlickEdit or VS Code - it uses content roots and source folders rather than 
# direct pattern-based file inclusion.
#
# APPROACHES CONSIDERED:
#
# 1. Basic Content Roots Only
#    - Simply add content roots for all directories containing cohort files
#    - Limitation: Would show ALL files in those directories, not just cohort files
#
# 2. Custom Scope-Based Approach
#    - Define a custom scope with patterns matching cohort files
#    - Use excludeFolder with includeFiles attribute to show only scope files
#    - Example: <excludeFolder url="file://$ROOT" includeFiles="$SCOPE"/>
#    - Limitation: Complex scope syntax, may not work consistently across versions
#
# 3. Pattern-Based Module Exclusion
#    - Use standard <excludePattern> in module file
#    - Show all files EXCEPT those explicitly excluded
#    - Limitation: Works by exclusion rather than inclusion (opposite of cohorts)
#
# 4. File System Scanning Approach (SELECTED)
#    - Scan each content root to find all files
#    - Keep those matching our cohort patterns, exclude everything else
#    - Generate excludePattern entries listing specific files to hide
#    - Advantage: Most precise control, works with IntelliJ's native features
#
# OUR IMPLEMENTATION:
#
# We chose approach #4 (file system scanning) for its precision and reliability.
# The implementation:
#
# 1. Gathers all content roots relevant to the workspace
# 2. For each content root, scans actual files on disk
# 3. Tests each file against cohort patterns (suffix, prefix, name)
# 4. Builds a list of files that don't match any cohort pattern
# 5. Generates XML with those files explicitly excluded
#
# This approach:
# - Provides exact control over which files are shown
# - Works reliably across IntelliJ versions
# - Properly handles mixed file types in the same directory
# - Aligns with our goal of precise cohort-based file organization
#
# NOTE: This implementation only excludes files that exist at generation time.
# If new files are added later, you'll need to regenerate the project files.
#==============================================================================
# Helper: escape text for XML attributes
# Helper: escape text for XML attributes
xml_escape() {
  local s="$1"
  s="${s//&/&amp;}"
  s="${s//</&lt;}"
  s="${s//>/&gt;}"
  s="${s//\"/&quot;}"
  s="${s//\'/&apos;}"
  printf '%s' "$s"
}

# Internal: generate IntelliJ .ipr/.iml files
zyeti_generate_intellij() {
  local out_dir="$1"
  local src_root="$2"
  local prefix="$3"

  # 1) Clean old files
  echo "YETIGEN: Clearing IntelliJ .{ipr,iml} prior: $out_dir"
  find "$out_dir" -maxdepth 1 -type f \( -name "*.ipr" -o -name "*.iml" \) -delete \
    || echo "WARNING: Failed to clear old IntelliJ files" >&2

  # ————————————————————————————————————————————————
  # Single filesystem scan for IntelliJ speedup
  declare -A ZYETI_FILES_IN_DIR
  while IFS= read -r -d '' f; do
    dir="${f%/*}"
    fname="${f##*/}"
    ZYETI_FILES_IN_DIR["$dir"]+="$fname"$'\n'
  done < <(find "$src_root" -type f -print0 2>/dev/null)
  # ————————————————————————————————————————————————

  # 2) Pre?gather content roots per workspace
  declare -A ALL_ROOTS
  for ws in "${!ZYETI_WORKSPACES[@]}"; do
    IFS='|' read -r _ cohort_list <<<"${ZYETI_WORKSPACES[$ws]}"

    # build quick lookup of desired cohorts
    declare -A WANT=()
    for cid in $cohort_list; do
      WANT["$cid"]=1
    done

    for pat_type in SUFFIX PREFIX NAME; do
      local arr_var="ZYETI_${pat_type}_PATTERNS"
      # collect all pattern IDs for this type
      eval "pids=(\"\${!$arr_var[@]}\")"
      # skip if none
      (( ${#pids[@]} == 0 )) && continue

      for pid in "${pids[@]}"; do
        # fetch the entry string
        eval "entry=\"\${$arr_var[\$pid]}\""
        IFS='|' read -r pat_cid repo_subpath _ _ <<<"$entry"
        [[ -n "${WANT[$pat_cid]}" ]] || continue

        local root="$src_root"
        [[ -n "$repo_subpath" ]] && root="$src_root/$repo_subpath"
        if [[ -d "$root" ]]; then
          ALL_ROOTS["$ws|$root"]=1
        else
          echo "WARNING: Content root does not exist: $root" >&2
        fi
      done
    done
  done

  # 3) For each workspace, emit .ipr and .iml
  for ws in "${!ZYETI_WORKSPACES[@]}"; do
    local prj="$out_dir/${prefix}-$ws.ipr"
    local mdl="$out_dir/${prefix}-$ws.iml"
    IFS='|' read -r _ cohort_list <<<"${ZYETI_WORKSPACES[$ws]}"

    # --- .ipr ---
    {
      printf '<?xml version="1.0" encoding="UTF-8"?>\n'
      printf '<project version="4">\n'
      printf '  <component name="ProjectModuleManager">\n'
      printf '    <modules>\n'
      printf '      <module fileurl="file://$PWD/%s" filepath="$PWD/%s"/>\n' \
             "$(xml_escape "${prefix}-$ws.iml")" "$(xml_escape "${prefix}-$ws.iml")"
      printf '    </modules>\n'
      printf '  </component>\n'
      printf '</project>\n'
    } > "$prj"

    # --- start .iml ---
    {
      printf '<?xml version="1.0" encoding="UTF-8"?>\n'
      printf '<module type="JAVA_MODULE" version="4">\n'
      printf '  <component name="NewModuleRootManager">\n'
    } > "$mdl"

    # rebuild WANT lookup
    declare -A WANT=()
    for cid in $cohort_list; do WANT["$cid"]=1; done

    # 4) process each content root
    for key in "${!ALL_ROOTS[@]}"; do
      IFS='|' read -r key_ws content_root <<<"$key"
      [[ "$key_ws" == "$ws" ]] || continue

      # a) gather all filenames in this root (from pre-scanned map)
      local -a all_files=()
      if [[ -n "${ZYETI_FILES_IN_DIR[$content_root]:-}" ]]; then
        mapfile -t all_files <<< "${ZYETI_FILES_IN_DIR[$content_root]}"
      fi

      # b) if empty, just include the folder
      if (( ${#all_files[@]} == 0 )); then
        {
          printf '    <content url="file://%s">\n' "$(xml_escape "$content_root")"
          printf '      <sourceFolder url="file://%s"/>\n'    "$(xml_escape "$content_root")"
          printf '    </content>\n'
        } >> "$mdl"
        continue
      fi

      # c) collect allowed suffixes/prefixes/names
      local -a allowed_suffixes=() allowed_prefixes=() allowed_names=()
      # suffix
      arr_var="ZYETI_SUFFIX_PATTERNS"; eval "pids=(\"\${!$arr_var[@]}\")"
      for pid in "${pids[@]}"; do
        eval "entry=\"\${$arr_var[\$pid]}\""
        IFS='|' read -r pat_cid repo_subpath suffix _ <<<"$entry"
        [[ -n "${WANT[$pat_cid]}" ]] || continue
        [[ "$content_root" == "$src_root/$repo_subpath" ]] || continue
        allowed_suffixes+=( "$suffix" )
      done
      # prefix
      arr_var="ZYETI_PREFIX_PATTERNS"; eval "pids=(\"\${!$arr_var[@]}\")"
      for pid in "${pids[@]}"; do
        eval "entry=\"\${$arr_var[\$pid]}\""
        IFS='|' read -r pat_cid repo_subpath prefix_val _ <<<"$entry"
        [[ -n "${WANT[$pat_cid]}" ]] || continue
        [[ "$content_root" == "$src_root/$repo_subpath" ]] || continue
        allowed_prefixes+=( "$prefix_val" )
      done
      # name
      arr_var="ZYETI_NAME_PATTERNS"; eval "pids=(\"\${!$arr_var[@]}\")"
      for pid in "${pids[@]}"; do
        eval "entry=\"\${$arr_var[\$pid]}\""
        IFS='|' read -r pat_cid repo_subpath name_val _ <<<"$entry"
        [[ -n "${WANT[$pat_cid]}" ]] || continue
        [[ "$content_root" == "$src_root/$repo_subpath" ]] || continue
        allowed_names+=( "$name_val" )
      done

      # d) compute disallowed (= all_files minus allowed)
      local -a disallowed=()
      for name in "${all_files[@]}"; do
        local keep=false
        for suf in "${allowed_suffixes[@]}";   do [[ "$name" == *"$suf"   ]] && { keep=true; break; } done
        for pre in "${allowed_prefixes[@]}";    do [[ "$name" == "$pre"*   ]] && { keep=true; break; } done
        for nm  in "${allowed_names[@]}";       do [[ "$name" == "$nm"     ]] && { keep=true; break; } done
        $keep || disallowed+=( "$name" )
      done

      # e) emit content + sourceFolder + excludePattern
      {
        printf '    <content url="file://%s">\n' "$(xml_escape "$content_root")"
        printf '      <sourceFolder url="file://%s"/>\n'    "$(xml_escape "$content_root")"
        if (( ${#disallowed[@]} )); then
          local -a esc=()
          for fn in "${disallowed[@]}"; do
            esc+=( "$(xml_escape "$fn")" )
          done
          local pat; pat="$(IFS=';'; echo "${esc[*]}")"
          printf '      <excludePattern pattern="%s"/>\n' "$pat"
        fi
        printf '    </content>\n'
      } >> "$mdl"
    done

    # close out
    {
      printf '  </component>\n'
      printf '</module>\n\n'
    } >> "$mdl"

    echo "YETIGEN: IntelliJ Project File generated:    $prj"
    echo "YETIGEN: IntelliJ Module File generated:     $mdl"
  done
}

# Public facade: emit all IDE configs for a given Space
function yeti_generate() {
  local spaces_dir="$1"
  local code_dir="$2"
  local prefix="$3"
  local rel_path_to_code="$4"

  test -z "$spaces_dir"           && { echo "ERROR: spaces_dir required";            exit 1; }
  test -z "$code_dir"             && { echo "ERROR: code_dir required";              exit 1; }
  test ! -d "$code_dir"           && { echo "ERROR: code_dir '$code_dir' not found"; exit 1; }
  test -z "$prefix"               && { echo "ERROR: prefix required";                exit 1; }
  test -z "$rel_path_to_code"     && { echo "ERROR: rel_path_to_code required";      exit 1; }

  mkdir -p "$spaces_dir" || { echo "ERROR: mkdir failed for $spaces_dir"; exit 1; }

  # Resolve absolute paths
  local abs_spaces_dir
  abs_spaces_dir=$(cd "$spaces_dir" && pwd)
  local abs_code_dir
  abs_code_dir=$(cd "$code_dir" && pwd)

  # Validate that rel_path_to_code resolves to code_dir
  local curdir
  curdir=$(pwd)
  cd "$abs_spaces_dir" || { echo "ERROR: cannot cd to $abs_spaces_dir"; exit 1; }
  local resolved_code_dir=""
  if [ -d "$rel_path_to_code" ]; then
    resolved_code_dir=$(cd "$rel_path_to_code" && pwd)
  fi
  cd "$curdir" || { echo "ERROR: cannot return to cwd"; exit 1; }

  if [ "$resolved_code_dir" != "$abs_code_dir" ]; then
    echo "ERROR: relative path '$rel_path_to_code' does not resolve to code_dir"
    echo "  - resolved to: $resolved_code_dir"
    echo "  - expected:    $abs_code_dir"
    exit 1
  fi

  # Kick off the internal generators
  zyeti_generate_slickedit  "$spaces_dir" "$rel_path_to_code" "$(basename "$code_dir")" "$prefix"
  zyeti_generate_vscode     "$spaces_dir" "$rel_path_to_code" "$prefix"
  zyeti_generate_intellij   "$spaces_dir" "$rel_path_to_code" "$prefix"
}

