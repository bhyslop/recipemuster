#!/bin/bash
# Execute make in a clean environment

set -euo pipefail

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

# Start Podman machine if it's not already running
podman machine start || echo "Podman probably running already, let's go on..."

# Run make in a clean environment with cherry picked variables from current environment.
#  This set was roughly determined necessary to enable podman to function.  Stripping the
#  environment revealed during container construction and invocation is a safety measure.
env -i \
  HOME="${HOME}"               \
  USER="${USER}"               \
  USERPROFILE="${USERPROFILE}" \
  PATH="${PATH}"               \
  APPDATA="${APPDATA}"         \
  TEMP="${TEMP}"               \
  TMP="${TMP}"                 \
  TERM="${TERM}"               \
  PODMAN_REMOTE=1              \
  PODMAN_USERNS=keep-id        \
  make -f rmc-console.mk "$OUTPUT_SYNC" -j "$JOBS" "$EXE" $ARGS
