#!/bin/bash
export BUD_LAUNCHER=".buk/launcher.rbw_workbench.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BUD_LAUNCHER}" "${0##*/}" "${@}"
