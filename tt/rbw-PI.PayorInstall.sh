#!/bin/bash
export BUD_LAUNCHER=".buk/launcher.rbk_Coordinator.sh"
export BUD_NO_LOG=1
export BUD_INTERACTIVE=1
exec "$(dirname "${BASH_SOURCE[0]}")/../${BUD_LAUNCHER}" "${0##*/}" "${@}"
