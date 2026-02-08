#!/bin/bash
export BUD_LAUNCHER=".buk/launcher.vow_workbench.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BUD_LAUNCHER}" "${0##*/}" "${@}"
