#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.cccw_workbench.sh"
export BURD_INTERACTIVE=1
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
