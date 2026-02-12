#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbw_workbench.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
