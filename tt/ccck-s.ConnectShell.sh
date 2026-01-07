#!/bin/bash
export BUD_LAUNCHER=".buk/launcher.cccw_workbench.sh"
export BUD_INTERACTIVE=1
exec "$(dirname "${BASH_SOURCE[0]}")/../${BUD_LAUNCHER}" "${0##*/}" "${@}"
