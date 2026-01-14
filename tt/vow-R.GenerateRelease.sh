#!/bin/bash
#
# VOK Generate Release - Build vvr binary for current platform
#
# Usage: vok-R.GenerateRelease.sh
#   Builds release binary, runs tests, copies to release dir, updates ledger

set -euo pipefail

SCRIPT_DIR="${BASH_SOURCE[0]%/*}"
PROJECT_ROOT="${SCRIPT_DIR}/.."

exec "${PROJECT_ROOT}/Tools/vok/vop_prepare_release.sh"
