#!/bin/bash
export BUD_LAUNCHER=".buk/launcher.rbk_Coordinator.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BUD_LAUNCHER}" "${0##*/}" "${@}"
