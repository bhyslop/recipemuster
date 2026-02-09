#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbk_Coordinator.sh"
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
