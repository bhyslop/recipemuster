#!/bin/bash

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

##########################################
# Makefile Bash Dispatch Shell Script
#
# Notes:
#  - Set verbose mode with MBD_VERBOSE=1 or MBD_VERBOSE=2 before command
#
# Commentary:
#   This script is the main dispatch entry point that leverages the 
#   mbd.utils.sh utility functions to set up the environment and execute
#   make with the necessary parameters, while maintaining a cleaner 
#   approach than the original implementation.

set -euo pipefail

# Initialize verbose level (used by utilities)
MBD_VERBOSE=${MBD_VERBOSE:-0}

# Source the utilities script
source "$(dirname "$0")/mbd.utils.sh" "$MBD_VERBOSE"
mbd_show "Starting dispatch script" "dispatch"

# Setup the environment (source and validate variables)
mbd_setup || { echo "ERROR: Environment setup failed" >&2; exit 1; }
mbd_show "Environment setup complete" "dispatch"

# Process the command-line arguments
mbd_process_args "$@" || { echo "ERROR: Argument processing failed" >&2; exit 1; }
mbd_show "Arguments processed" "dispatch"

# Generate the make command
make_cmd=$(mbd_gen_make_cmd)
mbd_show "Generated make command: $make_cmd" "dispatch"

# Log command to all log files
echo "command: $make_cmd" >> "$MBD_LOG_LAST" >> "$MBD_LOG_SAME" >> "$MBD_LOG_HIST"
echo "Git context: $MBD_GIT_CONTEXT" >> "$MBD_LOG_HIST"

mbd_show "Executing make command" "dispatch"

# Execute make with the generated command
set +e
STATUS_TMP="$MBD_TEMP_DIR/status-$$"
{ 
    eval "$make_cmd" 2>&1
    echo $? > "$STATUS_TMP"
    mbd_show "Make status: $(cat $STATUS_TMP)" "dispatch"
} | tee -a "$MBD_LOG_LAST" >(mbd_curate_same >> "$MBD_LOG_SAME") \
                           >(mbd_curate_hist >> "$MBD_LOG_HIST")
EXIT_STATUS=$(cat "$STATUS_TMP")
rm "$STATUS_TMP"
set -e

# Generate checksum for the log file
mbd_generate_checksum "$MBD_LOG_SAME" "$MBD_LOG_HIST"
mbd_show "Checksum generated" "dispatch"

mbd_show "Make completed with status: $EXIT_STATUS" "dispatch"

exit "$EXIT_STATUS"

