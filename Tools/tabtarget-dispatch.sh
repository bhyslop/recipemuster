#!/bin/bash
# Execute make in a clean environment

set -euo pipefail

# Path preparation
SCRIPT_PATH="/usr/local/bin:/usr/bin:/bin:/cygdrive/c/Program Files/RedHat/Podman"

# First parameter is the number of jobs we'll let MAKE use. 1 is default non-parallel
JOBS=$1
shift

# Second parameter is the rule to run, typically quite related to tabtarget invoking it
EXE=$1
shift

# All the rest of args are passed to make verbatim
ARGS="$@"

# Determine output synchronization
OUTPUT_SYNC=-Orecurse
if [ "$JOBS" == "1" ]; then
    OUTPUT_SYNC=-Oline
fi

# Preserve the TERM environment variable for pretty colors
CURRENT_TERM="${TERM:-xterm-256color}"

# Run make in a clean environment, but include TERM
env -i HOME="$HOME" PATH="$SCRIPT_PATH" TERM="$CURRENT_TERM" \
    make -f rmc-console.mk $OUTPUT_SYNC -j $JOBS $EXE $ARGS
