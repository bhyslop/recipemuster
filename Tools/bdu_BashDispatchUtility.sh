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
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Bash Dispatch Utility - Direct bash dispatch without make

set -euo pipefail

# Initialize verbose level
BDU_VERBOSE=${BDU_VERBOSE:-0}

# Source validation utilities
source "$(dirname "$0")/crgv.validate.sh"

# Utility function for verbose output
bdu_show() {
  test "$BDU_VERBOSE" != "1" || echo "BDUSHOW: $*"
}

# Enable trace mode if verbose level is 2
if [[ "$BDU_VERBOSE" == "2" ]]; then
  set -x
fi

# Source configuration and setup environment
bdu_setup() {
  bdu_show "Starting BDU setup"

  # Source main variables file
  local variables_file="./bdrv_Variables.sh"
  source "$variables_file"

  # Validate essential variables
  crgv_string "$variables_file" BDRV_STATION_FILE        1 256
  crgv_string "$variables_file" BDRV_LOG_LAST            1 256
  crgv_string "$variables_file" BDRV_LOG_EXT             1 32
  crgv_string "$variables_file" BDRV_TABTARGET_DIR       1 256
  crgv_string "$variables_file" BDRV_TABTARGET_DELIMITER 1 8
  crgv_string "$variables_file" BDRV_TEMP_ROOT_DIR       1 256
  crgv_string "$variables_file" BDRV_OUTPUT_ROOT_DIR     1 256
  crgv_string "$variables_file" BDRV_TOOLS_DIR           1 256
  crgv_string "$variables_file" BDRV_COORDINATOR_SCRIPT  1 256

  # Source station file
  bdu_show "Sourcing station file: $BDRV_STATION_FILE"
  source "$BDRV_STATION_FILE"

  # Validate station variables
  crgv_string "$BDRV_STATION_FILE" BDS_LOG_DIR 1 256

  # Generate timestamp
  BDU_NOW_STAMP=$(date +'%Y%m%d-%H%M%S')-$$-$((RANDOM % 1000))
  bdu_show "Generated timestamp: $BDU_NOW_STAMP"

  # Setup temporary directory
  BDU_TEMP_DIR="$BDRV_TEMP_ROOT_DIR/temp-$BDU_NOW_STAMP"
  mkdir -p "$BDU_TEMP_DIR"

  # Validate temporary directory
  if [[ ! -d "$BDU_TEMP_DIR" ]]; then
    echo "ERROR: Failed to create temporary directory: $BDU_TEMP_DIR" >&2
    return 1
  fi

  if [[ -n "$(find "$BDU_TEMP_DIR" -mindepth 1 -print -quit 2>/dev/null)" ]]; then
    echo "ERROR: Temporary directory is not empty: $BDU_TEMP_DIR" >&2
    return 1
  fi

  # Setup transcript file path
  BDU_TRANSCRIPT="${BDU_TEMP_DIR}/transcript.txt"

  # Setup output directory (fixed location, cleared on each run)
  BDU_OUTPUT_DIR="$BDRV_OUTPUT_ROOT_DIR/current"

  # Clear if exists, then create fresh
  if [[ -d "$BDU_OUTPUT_DIR" ]]; then
    bdu_show "Clearing existing output directory: $BDU_OUTPUT_DIR"
    rm -rf "$BDU_OUTPUT_DIR"
  fi
  mkdir -p "$BDU_OUTPUT_DIR"

  # Validate output directory
  if [[ ! -d "$BDU_OUTPUT_DIR" ]]; then
    echo "ERROR: Failed to create output directory: $BDU_OUTPUT_DIR" >&2
    return 1
  fi

  if [[ -n "$(find "$BDU_OUTPUT_DIR" -mindepth 1 -print -quit 2>/dev/null)" ]]; then
    echo "ERROR: Output directory is not empty: $BDU_OUTPUT_DIR" >&2
    return 1
  fi

  bdu_show "Output directory ready: $BDU_OUTPUT_DIR"

  # Get Git context
  BDU_GIT_CONTEXT=$(git describe --always --dirty --tags --long 2>/dev/null || echo "git-unavailable")
  bdu_show "Git context: $BDU_GIT_CONTEXT"

  # Export for child processes
  export BDU_TEMP_DIR
  export BDU_OUTPUT_DIR
  export BDU_NOW_STAMP
  export BDU_TRANSCRIPT

  return 0
}

# Process command-line arguments
bdu_process_args() {
  local target=$1
  shift

  bdu_show "Processing target: $target"

  # Extract tokens from tabtarget
  IFS="$BDRV_TABTARGET_DELIMITER" read -ra tokens <<< "$target"
  bdu_show "Split tokens: ${tokens[*]}"

  # Store primary command token
  BDU_COMMAND="${tokens[0]}"

  # Create tag for log files
  local tag="${tokens[0]}-${tokens[2]:-unknown}"

  # Setup log paths
  BDU_LOG_LAST="$BDS_LOG_DIR/$BDRV_LOG_LAST.$BDRV_LOG_EXT"
  BDU_LOG_SAME="$BDS_LOG_DIR/same-${tag}.$BDRV_LOG_EXT"
  BDU_LOG_HIST="$BDS_LOG_DIR/hist-${tag}-$BDU_NOW_STAMP.$BDRV_LOG_EXT"

  # Prepare log directories
  mkdir -p "$BDS_LOG_DIR"

  # Initialize log files
  > "$BDU_LOG_LAST"
  > "$BDU_LOG_SAME"
  > "$BDU_LOG_HIST"

  # Store target and extra arguments
  BDU_TARGET="$target"
  BDU_CLI_ARGS="$*"

  return 0
}

# Function to curate logs for the 'same' log file (normalized output)
bdu_curate_same() {
  # Convert to unix line endings, strip colors, normalize temp dir, remove VOLATILE lines
  sed -e 's/\r/\n/g'                           \
      -e '/^$/d'                               \
      -e 's/\x1b[\[][0-9;]*[a-zA-Z]//g'        \
      -e 's/\x1b[(][A-Z]//g'                   \
      -e "s|$BDU_TEMP_DIR|BDU_EPHEMERAL_DIR|g" \
      -e '/VOLATILE/d'
}

# Function to curate logs for the historical log file (with timestamps)
bdu_curate_hist() {
  while read -r line; do
    printf "[%s] %s\n" "$(date +"%Y-%m-%d %H:%M:%S")" "$line"
  done
}

# Generate and log checksum for a file
bdu_generate_checksum() {
  local file=$1
  local output_file=$2

  # Try multiple checksum commands (platform-dependent)
  local checksum=$(sha256sum            "$file" 2>/dev/null ||
                   openssl dgst -sha256 "$file" 2>/dev/null ||
                   echo "checksum-unavailable")

  echo "Same log checksum: $checksum" >> "$output_file"
  return 0
}

# Resolve color policy once at dispatch time and export BDU_COLOR (0/1)
bdu_resolve_color() {
  if [ -n "${NO_COLOR:-}" ]; then
    export BDU_COLOR=0
    return 0
  fi
  case "${BDU_COLOR:-auto}" in
    0|1)
      export BDU_COLOR
      ;;
    auto|*)
      if [ -t 1 ] && [ "${TERM:-}" != "dumb" ] && command -v tput >/dev/null 2>&1 && [ "$(tput colors 2>/dev/null || echo 0)" -gt 0 ]; then
          export BDU_COLOR=1
      else
          export BDU_COLOR=0
      fi
      ;;
  esac
}

# Main execution
bdu_main() {
  bdu_show "Starting BDU dispatch"

  # Decide color policy before stdout is piped
  bdu_resolve_color

  # Setup environment
  bdu_setup || (echo "ERROR: Environment setup failed" >&2 && exit 1)
  bdu_show "Environment setup complete"

  # Process arguments
  bdu_process_args "$@" || (echo "ERROR: Argument processing failed" >&2 && exit 1)
  bdu_show "Arguments processed"

  # Build coordinator command using configured script
  local coordinator_cmd="$BDRV_COORDINATOR_SCRIPT"
  bdu_show "Coordinator command: $coordinator_cmd $BDU_COMMAND $BDU_CLI_ARGS"

  # Log command to all log files
  echo "log files:   $BDU_LOG_LAST $BDU_LOG_SAME $BDU_LOG_HIST"
  echo "transcript:  ${BDU_TRANSCRIPT}"
  echo "output dir:  ${BDU_OUTPUT_DIR}"
  echo "command: $coordinator_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_LAST"
  echo "command: $coordinator_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_SAME"
  echo "command: $coordinator_cmd $BDU_COMMAND $BDU_CLI_ARGS" >> "$BDU_LOG_HIST"
  echo "Git context: $BDU_GIT_CONTEXT" >> "$BDU_LOG_HIST"

  bdu_show "Executing coordinator"

  # Execute coordinator with logging
  set +e
  zBDU_STATUS_FILE="$BDU_TEMP_DIR/status-$$"
  {
    "$coordinator_cmd" "$BDU_COMMAND" $BDU_CLI_ARGS 2>&1
    echo $? > "$zBDU_STATUS_FILE"
    bdu_show "Coordinator status: $(cat $zBDU_STATUS_FILE)"
  } | tee -a "$BDU_LOG_LAST" >(bdu_curate_same >> "$BDU_LOG_SAME") \
                             >(bdu_curate_hist >> "$BDU_LOG_HIST")
  zBDU_EXIT_STATUS=$(cat "$zBDU_STATUS_FILE")
  rm                     "$zBDU_STATUS_FILE"
  set -e

  # Generate checksum for the log file
  bdu_generate_checksum "$BDU_LOG_SAME" "$BDU_LOG_HIST"
  bdu_show "Checksum generated"

  bdu_show "BDU completed with status: $zBDU_EXIT_STATUS"

  exit "$zBDU_EXIT_STATUS"
}

bdu_main "$@"

# eof

