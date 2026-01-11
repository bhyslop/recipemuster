#!/bin/bash
#
# VVK Testbench - Run VVK tests
#
# Usage: vvk-T.RunTests.sh [test_name]
#   Run all tests or a specific test by name
#
# Environment:
#   BUT_VERBOSE=0  (default) minimal output
#   BUT_VERBOSE=1  show test names and section markers
#   BUT_VERBOSE=2  show trace messages
#   BUT_VERBOSE=3  enable bash -x tracing

set -euo pipefail

SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
PROJECT_ROOT="${SCRIPT_DIR}/.."

# Create fresh temp directory
z_temp_dir=$(mktemp -d)

# Cleanup on exit
cleanup() {
  rm -rf "${z_temp_dir}"
}
trap cleanup EXIT

# Run tests
exec "${PROJECT_ROOT}/Tools/vvk/vvt_cli.sh" "${z_temp_dir}" "${1:-}"
