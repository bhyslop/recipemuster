#!/bin/bash
export BURD_LAUNCHER=".buk/launcher.rbk_Coordinator.sh"
export BURD_NO_LOG=1
export BURD_INTERACTIVE=1
exec "$(dirname "${BASH_SOURCE[0]}")/../${BURD_LAUNCHER}" "${0##*/}" "${@}"
