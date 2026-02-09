#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbtg_testbench.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
