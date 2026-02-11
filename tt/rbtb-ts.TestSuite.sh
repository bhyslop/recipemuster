#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbtb_testbench.sh"
export BURD_NO_LOG=1
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
